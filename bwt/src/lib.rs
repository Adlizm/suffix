use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    ops::{AddAssign, Deref},
};

use word::{Symbol, Word};

pub struct Bwt<T> {
    bwt: Vec<Symbol<T>>,
}

impl<T> Bwt<T> {
    pub fn from_slice(data: &[T]) -> Self
    where
        T: Copy + Ord,
    {
        if data.len() == 0 {
            return Self {
                bwt: vec![Symbol::Terminal],
            };
        }

        let sa = suffix_array::SuffixArray::from_slice(data);
        let saa = sa.array();

        let n = data.len();

        // The BWT is the last column of the sorted rotation matrix
        let bwt: Vec<Symbol<T>> = saa
            .iter()
            .map(|&i| data.symbol_at((i + n - 1) % n))
            .collect();

        Self { bwt }
    }

    pub fn into_inner(self) -> Vec<Symbol<T>> {
        self.bwt
    }

    pub fn revert<S>(self) -> S
    where
        T: Copy + Ord + Hash,
        S: FromIterator<T>,
    {
        if self.bwt.len() == 1 {
            return S::from_iter(std::iter::empty());
        }

        let n = self.bwt.len();
        let mut origin = 0usize;

        // Count occurrences of each byte
        let mut count: BTreeMap<_, usize> = BTreeMap::new();
        for (index, &b) in self.bwt.iter().enumerate() {
            count.entry(b).or_default().add_assign(1);
            if b.is_terminal() {
                origin = index;
            }
        }

        // Cumulative counts (first occurrence of each byte in first column)
        let mut first_occurrence: HashMap<_, usize> = HashMap::new();
        let mut sum = 0;
        for (&c, count) in &count {
            first_occurrence.insert(c, sum);
            sum += count;
        }

        // Build LF-mapping: for each position in last column, find position in first column
        let mut lf = vec![0usize; n];
        let mut occ_count: HashMap<_, usize> = HashMap::new();
        for i in 0..n {
            let b = self.bwt[i];
            lf[i] = first_occurrence.get(&b).unwrap() + occ_count.get(&b).unwrap();

            occ_count.entry(b).or_default().add_assign(1);
        }

        // Reconstruct by following LF-mapping from orig_idx
        let mut result = Vec::<T>::with_capacity(n - 1);

        let letters = result.spare_capacity_mut();
        let mut idx = lf[origin];
        for i in (0..n - 1).rev() {
            let Symbol::Char(letter) = self.bwt[idx] else {
                unreachable!("bwt has a uniqui Symbol::Terminal that aready skip")
            };

            letters[i].write(letter);
            idx = lf[idx];
        }

        // Filled in the previous loop
        unsafe { result.set_len(n - 1) };

        S::from_iter(result)
    }
}

impl<T> Deref for Bwt<T> {
    type Target = [Symbol<T>];

    fn deref(&self) -> &Self::Target {
        &self.bwt
    }
}
