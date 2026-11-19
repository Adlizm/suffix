use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    ops::AddAssign,
};

use bwt::Bwt;
use wavelet_tree::WaveletTree;
use word::Symbol;

pub(crate) struct Count<T> {
    acc: usize,
    next: Option<Symbol<T>>,
}

pub struct FMIndex<T> {
    wavelet_tree: WaveletTree<Symbol<T>>,
    occ_count: HashMap<Symbol<T>, Count<T>>,
}

impl<T> FMIndex<T>
where
    T: Copy + Ord + Hash,
{
    pub fn from_slice(word: &[T]) -> Self {
        let bwt = Bwt::from_slice(word).into_inner();
        let wavelet_tree = WaveletTree::from(bwt.as_slice());

        let mut count = BTreeMap::new();
        for &c in word {
            count.entry(c).or_insert(0usize).add_assign(1);
        }

        let mut sum = 0;
        let mut prev = Symbol::Terminal;

        let mut occ_count: HashMap<Symbol<T>, Count<T>> = HashMap::new();

        for (c, ccount) in count {
            occ_count.get_mut(&prev).unwrap().next = Some(Symbol::Char(c));

            let count = Count {
                acc: sum,
                next: None,
            };
            occ_count.insert(Symbol::Char(c), count);

            sum += ccount;
            prev = Symbol::Char(c);
        }

        Self {
            wavelet_tree,
            occ_count,
        }
    }

    pub fn contains(&self, pattern: &[T]) -> bool {
        if pattern.len() == 0 {
            return true;
        }

        let mut index = pattern.len() - 1;
        let mut c = pattern[index];
        let (mut sp, mut ep) = if let Some(data) = self.occ_count.get(&Symbol::Char(c)) {
            let sp = data.acc + 1;
            let ep = if let Some(next) = data.next {
                self.occ_count.get(&next).unwrap().acc
            } else {
                self.wavelet_tree.len()
            };

            (sp, ep)
        } else {
            return false;
        };

        while (ep >= sp) && index >= 1 {
            c = pattern[index - 1];

            let cc = self.occ_count.get(&Symbol::Char(c)).unwrap().acc;

            sp = cc + self.wavelet_tree.rank(Symbol::Char(c), sp - 1) + 1;
            ep = cc + self.wavelet_tree.rank(Symbol::Char(c), ep);

            index -= 1;
        }

        ep >= sp
    }
}
