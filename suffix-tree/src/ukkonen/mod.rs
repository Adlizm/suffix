use crate::ukkonen::data::{NodeIndex, SuffixTreeData};
use crate::word::{Symbol, Word};

mod builder;
pub mod data;

#[derive(Debug)]
pub struct SuffixTree<'a, T: Word + ?Sized> {
    data: SuffixTreeData<T::Alphabet>,
    word: &'a T,
}

impl<'a, T: Word + ?Sized> SuffixTree<'a, T> {
    pub fn new(word: &'a T) -> Self {
        builder::SuffixTreeBuilder::build_tree(word)
    }

    pub fn data(&self) -> &SuffixTreeData<T::Alphabet> {
        &self.data
    }

    pub fn word(&self) -> &'a T {
        self.word
    }

    pub fn contains(&self, pattern: &T) -> bool {
        let mut current = NodeIndex::root();
        let mut pattern = pattern.symbols();

        while let Some(c) = pattern.next() {
            let len = self.word.size_of(c);

            if let Some((node_index, node)) = self.data.get_edge(current, Symbol::Char(c)) {
                let mut suffix = (node.start + len..node.end)
                    .into_iter()
                    .map(|i| self.word.get(i));

                while let Some(Symbol::Char(s)) = suffix.next() {
                    if let Some(p) = pattern.next() {
                        if s != p {
                            return false;
                        }
                    } else {
                        return true;
                    }
                }

                current = node_index;
            } else {
                return false;
            }
        }

        true
    }
}

pub type SuffixTreeStr<'a> = SuffixTree<'a, str>;

impl std::fmt::Display for SuffixTreeStr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        fn format_node(
            node_index: NodeIndex,
            t: &SuffixTreeStr<'_>,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            if node_index != NodeIndex::root() {
                let node = t.data.get_node(node_index);

                if node.end - node.start == 0 {
                    f.write_char('$')?;
                } else {
                    f.write_str(&t.word[node.start..node.end])?;
                }
            }

            let Some(edges) = t.data.get_edges(node_index) else {
                return Ok(());
            };

            f.write_char('(')?;
            let mut first = true;
            for (_, node_index) in edges {
                if !first {
                    f.write_str(", ")?;
                } else {
                    first = false;
                }

                format_node(node_index, t, f)?;
            }
            f.write_char(')')
        }

        format_node(NodeIndex::root(), self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banana() {
        let tree = SuffixTree::new("banana");
        assert_eq!(tree.to_string(), "($, a($, na($, na)), banana, na($, na))")
    }

    #[test]
    fn test_abcabxabcd() {
        let tree = SuffixTree::new("abcabx");
        assert_eq!(tree.to_string(), "($, ab(cabx, x), b(cabx, x), cabx, x)");

        let tree = SuffixTree::new("abcabxa");
        assert_eq!(
            tree.to_string(),
            "($, a($, b(cabxa, xa)), b(cabxa, xa), cabxa, xa)"
        );

        let tree = SuffixTree::new("abcabxab");
        assert_eq!(
            tree.to_string(),
            "($, ab($, cabxab, xab), b($, cabxab, xab), cabxab, xab)"
        );

        let tree = SuffixTree::new("abcabxabd");
        assert_eq!(
            tree.to_string(),
            "($, ab(cabxabd, d, xabd), b(cabxabd, d, xabd), cabxabd, d, xabd)"
        );

        let tree = SuffixTree::new("abcabxabc");
        assert_eq!(
            tree.to_string(),
            "($, ab(c($, abxabc), xabc), b(c($, abxabc), xabc), c($, abxabc), xabc)"
        );

        let tree = SuffixTree::new("abcabxabcd");
        assert_eq!(
            tree.to_string(),
            "($, ab(c(abxabcd, d), xabcd), b(c(abxabcd, d), xabcd), c(abxabcd, d), d, xabcd)"
        );
    }
}
