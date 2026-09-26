use suffix_tree::{ukkonen::SuffixTree, word::Word};

mod from_tree;
mod skew;

mod lcp;

#[derive(Debug, Clone, PartialEq)]
pub struct SuffixArray<'a, T>
where
    T: ?Sized,
{
    word: &'a T,
    array: Vec<usize>,
    lcp: Vec<usize>,
}

impl<'a> SuffixArray<'a, str> {
    pub fn from_ascii_str(word: &'a str) -> Self {
        assert!(word.is_ascii());

        let mut array = vec![0; word.len() + 1];

        array[0] = word.len();
        skew::SuffixArrayBuilder::build_in_place(word.as_bytes(), &mut array[1..]);

        let lcp = lcp::compute_lcp(word.as_bytes(), &array);

        Self { word, array, lcp }
    }
}

impl<'a, T: Word + ?Sized> SuffixArray<'a, T> {
    pub fn new(word: &'a T) -> Self {
        SuffixTree::new(word).into()
    }

    pub fn word(&self) -> &T {
        self.word
    }

    pub fn array(&self) -> &[usize] {
        &self.array
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
    }

    #[test]
    fn test_processing() {
        let array = SuffixArray::from_ascii_str("processing");

        assert_eq!(array.array(), &[10, 3, 4, 9, 7, 8, 2, 0, 1, 6, 5]);
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
