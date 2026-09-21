mod from_tree;

#[derive(Debug, Clone, PartialEq)]
pub struct SuffixArray<'a, T>
where
    T: ?Sized,
{
    word: &'a T,
    data: Box<[usize]>,
}

impl<'a, T: ?Sized> SuffixArray<'a, T> {
    pub fn new(_word: &'a str) -> Self {
        todo!()
    }

    pub fn word(&self) -> &T {
        self.word
    }

    pub fn data(&self) -> &[usize] {
        &self.data
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

        assert_eq!(array.data(), &[6, 5, 3, 1, 0, 4, 2]);
    }
}
