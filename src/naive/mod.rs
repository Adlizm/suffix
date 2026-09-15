use std::fmt::{Debug, Display, Write};

mod node;
use node::Nodes;

#[derive(Debug, Clone)]
pub struct SuffixTree<'a> {
    src: &'a str,
    root: Nodes<'a>,
}

impl<'a> Display for SuffixTree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_inners(nodes: &Nodes, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_char('(')?;
            let mut first = true;

            for node in &nodes.0 {
                if !first {
                    f.write_str(", ")?;
                } else {
                    first = false;
                }

                if node.suffix.len() == 0 {
                    f.write_char('$')?;
                } else {
                    Debug::fmt(&node.suffix, f)?;
                }

                if node.nodes.0.len() > 0 {
                    fmt_inners(&node.nodes, f)?;
                }
            }

            f.write_char(')')
        }

        fmt_inners(&self.root, f)
    }
}

impl<'a> SuffixTree<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut root = Nodes::empty();

        let mut offset = 0;
        root.add_suffix(src);

        for c in src.chars() {
            offset += c.len_utf8();
            root.add_suffix(&src[offset..]);
        }

        Self { src, root }
    }

    pub fn src(&self) -> &str {
        self.src
    }

    pub fn substring_count(&self, substring: &str) -> usize {
        self.root.substring_count(substring)
    }

    pub fn contains(&self, substring: &str) -> bool {
        self.root.find_node(substring).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find() {
        let tree = SuffixTree::new("banana");
        println!("{}", tree);

        assert!(tree.contains("ban"));
        assert_eq!(tree.substring_count("ana"), 2);
    }
}
