use suffix_tree::{
    ukkonen::{SuffixTree, data::NodeIndex},
    word::Word,
};

use crate::SuffixArray;

impl<'a, T> From<SuffixTree<'a, T>> for SuffixArray<'a, T>
where
    T: ?Sized + Word,
{
    fn from(tree: SuffixTree<'a, T>) -> Self {
        let word = tree.word();
        let len = word.symbols().count() + 1;
        let mut array = Vec::with_capacity(len);

        fn deep<'a, T>(
            node: NodeIndex,
            data: &mut Vec<usize>,
            tree: &SuffixTree<'a, T>,
            suffix_len: usize,
        ) where
            T: ?Sized + Word,
        {
            if let Some(edges) = tree.data().get_edges(node) {
                for (_, index) in edges {
                    let node = tree.data().get_node(index);

                    deep(index, data, tree, suffix_len - (node.end - node.start));
                }
            } else {
                data.push(suffix_len);
            }
        }

        deep(NodeIndex::root(), &mut array, &tree, word.size());

        let lcp = super::lcp::compute_lcp(word, &array);

        SuffixArray { word, array, lcp }
    }
}
