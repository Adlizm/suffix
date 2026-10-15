use std::cmp::Ordering;

use suffix_tree::word::Word;

mod from_tree;
mod skew;

#[derive(Debug, Clone, PartialEq)]
pub struct SuffixArray<'a, T>
where
    T: ?Sized,
{
    word: &'a T,
    array: Vec<usize>,
}

impl<'a, T: Copy + Ord> SuffixArray<'a, [T]> {
    pub fn from_slice(word: &'a [T]) -> Self {
        let mut array = vec![0; word.len() + 1];

        array[0] = word.len();
        skew::SuffixArrayBuilder::build_in_place(word, &mut array[1..]);

        Self { word, array }
    }
}

impl<'a> SuffixArray<'a, str> {
    pub fn from_ascii_str(word: &'a str) -> Self {
        assert!(word.is_ascii());

        let mut array = vec![0; word.len() + 1];

        array[0] = word.len();
        skew::SuffixArrayBuilder::build_in_place(word.as_bytes(), &mut array[1..]);

        Self { word, array }
    }
}

impl<'a, T: Word + ?Sized> SuffixArray<'a, T> {
    pub fn word(&self) -> &T {
        self.word
    }

    pub fn array(&self) -> &[usize] {
        &self.array
    }

    /// Naive implementation of `contains` that scans the suffix array O(|p| * log|n|).
    pub fn contains(&self, pattern: &T) -> bool {
        let mut left = 0;
        let mut right = self.array.len() - 1;

        'ineer: while left <= right {
            let mid = (left + right) / 2;
            let suffix = self.word.suffix(self.array[mid]);

            for (s, p) in suffix.symbols().zip(pattern.symbols()) {
                match s.cmp(&p) {
                    Ordering::Equal => continue,
                    Ordering::Less => {
                        left = mid + 1;
                        continue 'ineer;
                    }
                    Ordering::Greater => {
                        right = mid - 1;
                        continue 'ineer;
                    }
                }
            }

            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::SuffixArray;
    use suffix_tree::ukkonen::SuffixTree;

    #[test]
    fn test_banana() {
        let tree = SuffixTree::new("banana");
        let array: SuffixArray<_> = tree.into();

        assert_eq!(array.array(), &[6, 5, 3, 1, 0, 4, 2]);

        assert!(array.contains("ana"));
        assert!(array.contains("nana"));
    }

    #[test]
    fn test_processing() {
        let array = SuffixArray::from_ascii_str("processing");

        assert_eq!(array.array(), &[10, 3, 4, 9, 7, 8, 2, 0, 1, 6, 5]);

        assert!(array.contains("cessing"));
        assert!(array.contains("roces"));
    }

    #[test]
    fn test_equals() {
        fn test_tree_and_array(word: &str) {
            let a = SuffixArray::from(SuffixTree::new(word));
            let b = SuffixArray::from_ascii_str(word);

            assert_eq!(a.array(), b.array());
        }

        test_tree_and_array("banana");
        test_tree_and_array("processing");
        test_tree_and_array("mississippi");

        test_tree_and_array("aaaaaa");
        test_tree_and_array("abcabc");
    }
}
