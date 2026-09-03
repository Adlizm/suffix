use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub(crate) struct Nodes<'a>(pub(crate) Vec<Node<'a>>);

#[derive(Debug, Clone)]
pub(crate) struct Node<'a> {
    pub(crate) suffix: &'a str,

    pub(crate) nodes: Nodes<'a>,
}

impl<'a> Node<'a> {
    pub(crate) fn count_leaves(&self) -> usize {
        if self.nodes.0.len() == 0 {
            1
        } else {
            self.nodes.0.iter().map(|inner| inner.count_leaves()).sum()
        }
    }
}

impl<'a> Nodes<'a> {
    pub(crate) fn empty() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn add_suffix(&mut self, suffix: &'a str) {
        let mut suffix_chars = suffix.chars();
        let Some(ssc) = suffix_chars.next() else {
            return self.0.push(Node {
                suffix: "",
                nodes: Nodes::empty(),
            });
        };

        for node in self.0.iter_mut() {
            let mut suffix_charts = suffix_chars.clone();
            let mut node_suffix_chars = node.suffix.chars();

            if let Some(nsc) = node_suffix_chars.next()
                && nsc == ssc
            {
                let mut match_len = nsc.len_utf8();

                while let Some(nsc) = node_suffix_chars.next()
                    && let Some(ssc) = suffix_charts.next()
                    && nsc == ssc
                {
                    match_len += ssc.len_utf8();
                }

                if match_len == node.suffix.len() {
                    return node.nodes.add_suffix(&suffix[match_len..]);
                } else {
                    debug_assert!(match_len < node.suffix.len());

                    *node = Node {
                        suffix: &node.suffix[0..match_len],

                        nodes: Nodes(vec![
                            Node {
                                suffix: &node.suffix[match_len..node.suffix.len()],

                                nodes: std::mem::replace(&mut node.nodes, Nodes::empty()),
                            },
                            Node {
                                suffix: &suffix[match_len..],

                                nodes: Nodes::empty(),
                            },
                        ]),
                    };

                    return;
                }
            }
        }

        self.0.push(Node {
            suffix: suffix,
            nodes: Nodes::empty(),
        });
    }

    pub(crate) fn substring_count(&self, substring: &str) -> usize {
        if let Some(node) = self.find_node(substring) {
            node.count_leaves()
        } else {
            0
        }
    }

    pub(crate) fn find_node(&self, mut suffix: &str) -> Option<&Node<'a>> {
        let mut nodes = &self.0;
        loop {
            let Some(node) = nodes.iter().find(|node| {
                let find = &node.suffix[..node.suffix.len().min(suffix.len())];

                suffix.starts_with(find)
            }) else {
                return None;
            };

            match suffix.len().cmp(&node.suffix.len()) {
                Ordering::Equal | Ordering::Less => return Some(node),
                Ordering::Greater => {
                    suffix = &suffix[node.suffix.len()..];
                    nodes = &node.nodes.0;
                }
            }
        }
    }
}
