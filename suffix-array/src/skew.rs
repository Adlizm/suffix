use std::collections::BTreeMap;

use chek::debug_unreachable;
use suffix_tree::word::{Symbol, Word};

pub struct SuffixArrayBuilder<'a, T> {
    word: &'a [T],
    sa: &'a mut [usize],

    sa0: Vec<usize>,
    ra12: Vec<usize>,
    sa12: Vec<usize>,
}

impl<'a, T> SuffixArrayBuilder<'a, T>
where
    T: Ord + Copy,
{
    pub fn build_in_place(word: &'a [T], sa: &'a mut [usize]) {
        assert!(sa.len() >= word.len());

        let n2 = word.len() / 3;
        let n1 = (word.len() + 1) / 3;
        let n0 = (word.len() + 2) / 3;
        let n12 = n1 + n2;

        let mut this = Self {
            word,
            sa,

            sa12: Vec::with_capacity(n12),
            ra12: Vec::with_capacity(n12),

            sa0: Vec::with_capacity(n0),
        };

        this.create_array12();

        this.create_array0();

        this.merge();
    }

    fn create_array12(&mut self) {
        #[rustfmt::skip]
        let Self { word, sa12, ra12, .. } = self;

        for index in 0..word.len() {
            if index % 3 != 0 {
                sa12.push(index);
            }
        }

        let n12 = sa12.len();
        let n1 = (word.len() + 1) / 3;

        let mut count = Default::default();
        radix_sort(&word[2..], sa12, &mut count);
        radix_sort(&word[1..], sa12, &mut count);
        radix_sort(&word[0..], sa12, &mut count);

        drop(count);

        // Safety ra12 is initialized in the next loop
        unsafe { ra12.set_len(n12) };

        let mut letter = 0;
        let mut prevc = (None, None, None);
        for &index in sa12.iter() {
            let mut suffix = word.suffix(index).symbols();
            let nextc = (suffix.next(), suffix.next(), suffix.next());
            if nextc != prevc {
                letter += 1;
                prevc = nextc;
            }

            match index % 3 {
                1 => ra12[index / 3] = letter - 1,
                2 => ra12[index / 3 + n1] = letter - 1,
                _ => debug_unreachable!(
                    "Since index is never divide by 3 when call this function with `word12`"
                ),
            }
        }

        if letter < n12 {
            SuffixArrayBuilder::build_in_place(&ra12, sa12);

            // rank array set for `ra12`
            for i in 0..n12 {
                ra12[sa12[i]] = i;
            }
        } else {
            // suffix array set for `sa12` (inverse of `ra12` that is a rank array)
            for i in 0..n12 {
                sa12[ra12[i]] = i;
            }
        }

        for i in 0..n12 {
            sa12[i] = if sa12[i] < n1 {
                3 * sa12[i] + 1
            } else {
                2 + 3 * (sa12[i] - n1)
            };
        }
    }

    fn create_array0(&mut self) {
        #[rustfmt::skip]
        let Self { word, sa0, sa12, .. } = self;

        if word.len() % 3 == 1 {
            // last element is in s0
            sa0.push(word.len() - 1);
        }

        for &mut index in sa12 {
            if index % 3 == 1 {
                sa0.push(index - 1);
            }
        }
        radix_sort(word, sa0, &mut Default::default());
    }

    fn merge(&mut self) {
        #[rustfmt::skip]
        let Self { word, sa, sa0, sa12, ra12 } = self;

        assert_eq!(word.len(), sa0.len() + sa12.len());

        let mut index0 = 0;
        let mut index12 = 0;

        let n = word.len();
        let n1 = (n + 1) / 3;

        let rank = |index: usize| -> usize {
            if index % 3 == 1 {
                ra12.get(index / 3).copied().unwrap_or(0)
            } else {
                ra12.get(index / 3 + n1).copied().unwrap_or(0)
            }
        };
        let word = |index: usize| word.symbol_at(index);

        for k in 0..n {
            let i = sa0[index0];
            let j = sa12[index12];

            let next_in0 = if j % 3 == 1 {
                (word(i), rank(i + 1)) < (word(j), rank(j + 1))
            } else {
                (word(i), word(i + 1), rank(i + 2)) < (word(j), word(j + 1), rank(j + 2))
            };

            if next_in0 {
                sa[k] = i;
                index0 += 1;

                if index0 == sa0.len() {
                    for k in k + 1..n {
                        sa[k] = sa12[index12];
                        index12 += 1;
                    }
                    break;
                }
            } else {
                sa[k] = j;
                index12 += 1;

                if index12 == sa12.len() {
                    for k in k + 1..n {
                        sa[k] = sa0[index0];
                        index0 += 1;
                    }
                    break;
                }
            }
        }
    }
}

type Count<T> = BTreeMap<Symbol<T>, Vec<usize>>;

fn radix_sort<T>(word: &[T], s: &mut Vec<usize>, count: &mut Count<T>)
where
    T: Ord + Copy,
{
    for index in s.drain(..) {
        let key = word
            .get(index)
            .copied()
            .map_or(Symbol::Terminal, Symbol::Char);

        count.entry(key).or_default().push(index);
    }

    for (_, values) in count.iter_mut() {
        for value in values.drain(..) {
            s.push(value);
        }
    }
}
