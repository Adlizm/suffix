use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    ops::AddAssign,
};

use word::{Symbol, UniformeWord, Word};

pub struct Bwt<T: UniformeWord + ?Sized> {
    bwt: Vec<Symbol<T::Alphabet>>,
}

impl<T> Bwt<[T]>
where
    T: Copy + Ord + Hash,
{
    /// Compute the BWT of the input data.
    pub fn from_slice(data: &[T]) -> Self {
        if data.len() == 0 {
            return Self {
                bwt: vec![Symbol::Terminal],
            };
        }

        let n = data.len();
        let sa = suffix_array::SuffixArray::from_slice(data);
        let saa = sa.array();

        // The BWT is the last column of the sorted rotation matrix
        let bwt: Vec<Symbol<T>> = saa
            .iter()
            .map(|&i| data.symbol_at((i + n - 1) % n))
            .collect();

        Self { bwt }
    }

    pub fn revert(self) -> Vec<T> {
        if self.bwt.is_empty() {
            return vec![];
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

        result
    }
}
