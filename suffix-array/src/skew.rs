use std::collections::BTreeMap;

use chek::debug_unreachable;
use suffix_tree::word::{Symbol, Word};

pub struct SuffixArrayBuilder<'a, T> {
    word: &'a [T],
    sa: &'a mut [usize],

    w12: Vec<usize>,
    isa12: Vec<usize>,

    w0: Vec<usize>,
    isa0: Vec<usize>,
}

impl<'a, T> SuffixArrayBuilder<'a, T>
where
    T: Ord + Copy,
{
    pub fn build_in_place(word: &'a [T], sa: &'a mut [usize]) {
        assert_eq!(sa.len(), word.len());

        let n2 = word.len() / 3;
        let n1 = (word.len() + 1) / 3;
        let n12 = n1 + n2;

        let n0 = word.len() - n12;

        let mut this = Self {
            word,
            sa,

            w12: Vec::with_capacity(n12),
            isa12: Vec::with_capacity(n12),

            w0: Vec::with_capacity(n0),
            isa0: Vec::with_capacity(n0),
        };

        this.initialize();

        this.create_inverse_array12();

        this.create_inverse_array0();

        this.merge();
    }

    fn initialize(&mut self) {
        // Creating s12
        let mut i = 0;
        for (index, c) in self.word.symbols().enumerate() {
            if index % 3 != 0 {
                self.w12.push(i);
                self.isa12.push(0);
            }
            i += self.word.size_of(c);
        }
    }

    fn create_inverse_array12(&mut self) {
        let Self {
            word, w12, isa12, ..
        } = self;

        let n12 = w12.len();

        let mut count = Default::default();
        radix_sort(&word[2..], w12, isa12, &mut count);
        radix_sort(&word[1..], w12, isa12, &mut count);
        radix_sort(&word[0..], w12, isa12, &mut count);

        drop(count);

        let mut letter = 0;
        let mut prevc = (None, None, None);
        for &index in isa12.iter() {
            let mut suffix = word.suffix(index).symbols();
            let nextc = (suffix.next(), suffix.next(), suffix.next());
            if nextc != prevc {
                letter += 1;
                prevc = nextc;
            }

            match index % 3 {
                1 => w12[index / 3] = letter - 1,
                2 => w12[index / 3 + n12 / 2] = letter - 1,
                _ => debug_unreachable!(
                    "Since index is never divide by 3 when call this function with `word12`"
                ),
            }
        }

        if letter < n12 {
            SuffixArrayBuilder::build_in_place(w12, isa12);

            for i in 0..n12 {
                w12[isa12[i]] = i;
            }
        } else {
            for i in 0..n12 {
                isa12[w12[i]] = i;
            }
        }

        dbg!(&w12);
        dbg!(&isa12);
    }
    fn create_inverse_array0(&mut self) {
        #[rustfmt::skip]
        let Self { word, w0, w12, isa0, isa12, .. } = self;

        let mut j = 0;
        for i in 0..w12.len() {
            if isa12[i] < w0.len() {
                w0[j] = 3 * isa12[i];
                j += 1;
            }
        }
        radix_sort(word, w0, isa0, &mut Default::default());
    }

    fn merge(&mut self) {
        #[rustfmt::skip]
        let Self { word, sa, w0, w12, isa0, isa12 } = self;

        assert_eq!(word.len(), w0.len() + w12.len());

        let mut index0 = 0;
        let mut index12 = 0;

        fn get_i(index12: usize, w0: &[usize], s12: &[usize]) -> usize {
            if s12[index12] < w0.len() {
                // in s1
                s12[index12] * 3 + 1
            } else {
                // in s2
                (s12[index12] - w0.len()) * 3 + 2
            }
        }

        for k in 0..word.len() {
            let i = get_i(index12, w0, isa12);
            let j = isa0[index0];

            dbg!(i, j);

            let next_in0 = if isa12[index12] < w0.len() {
                (word[i], w12[isa12[index12]]) < (word[j], w12[j / 3])
            } else {
                (word[i], word[i + 1], w12[isa12[index12] - word.len() + 1])
                    < (word[j], word[j + 1], w12[j / 3 + w0.len()])
            };

            if !next_in0 {
                sa[k] = i;
                index12 += 1;

                if index12 == w12.len() {
                    for k in k + 1..word.len() {
                        sa[k] = isa0[index0];
                        index0 += 1;
                    }
                    break;
                }
            } else {
                sa[k] = j;
                index0 += 1;

                if index0 == w0.len() {
                    for k in k + 1..word.len() {
                        sa[k] = get_i(index12, w0, isa12);
                        index12 += 1;
                    }
                    break;
                }
            }
        }
    }
}

type Count<T> = BTreeMap<Symbol<T>, Vec<usize>>;

fn radix_sort<T>(word: &[T], s: &[usize], array: &mut [usize], count: &mut Count<T>)
where
    T: Ord + Copy,
{
    for &index in s {
        let key = word
            .get(index)
            .copied()
            .map_or(Symbol::Terminal, Symbol::Char);

        count.entry(key).or_default().push(index);
    }

    let mut index = 0;
    for (_, values) in count.iter_mut() {
        for value in values.drain(..) {
            array[index] = value;
            index += 1;
        }
    }
}
