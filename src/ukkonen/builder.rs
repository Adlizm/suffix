use crate::ukkonen::data::{Node, NodeIndex, SuffixTreeData};

pub(super) struct SuffixTreeBuilder<'a> {
    pub(super) word: &'a str,
    pub(super) position: usize,
    pub(super) remaining: usize,

    pub(super) active: Active,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Active {
    Node {
        node: NodeIndex,
    },
    Edge {
        node: NodeIndex,
        edge: char,
        len: usize,
    },
}

pub const TERMINAL: char = '$';

impl<'a> SuffixTreeBuilder<'a> {
    pub fn build_tree(word: &'a str) -> super::SuffixTree<'a> {
        let mut data = SuffixTreeData::new_with_root();
        let mut builder = Self {
            word,
            active: Active::Node {
                node: NodeIndex::root(),
            },
            remaining: 0,
            position: 0,
        };

        for c in word.chars() {
            let _ = builder.step(c, &mut data);
            builder.position += c.len_utf8();
        }

        let _ = builder.step(TERMINAL, &mut data);

        debug_assert!(builder.remaining == 0);

        super::SuffixTree { data, word }
    }

    fn step(&mut self, c: char, data: &mut SuffixTreeData) -> Option<NodeIndex> {
        match self.active {
            Active::Edge { node, edge, len } => {
                let (edge_index, edgen) = {
                    if let Some(edgen) = data.get_edge_mut(node, edge) {
                        edgen
                    } else {
                        // This is a reachable state?? need to be handled?
                        // This represents a Active::Edge that still is not present (handle like Case B ?),
                        todo!();
                    }
                };

                let other_offset = edgen.start + len;
                let Some(other) = self.word[other_offset..edgen.end].chars().next() else {
                    // This is a reachable state?? need to be handled?
                    self.active = Active::Node { node: edge_index };

                    return self.step(c, data);
                };

                if other == c {
                    // Case D
                    self.remaining += 1;

                    self.active = Active::Edge {
                        node,
                        edge,
                        len: len + other.len_utf8(),
                    };

                    None
                } else {
                    // Case E
                    let created_node = self.create_node_split_edge(data);
                    self.remaining -= 1;

                    let mut link = data.get_node(node).link.unwrap_or(NodeIndex::root());

                    if self.remaining > 0 {
                        let next = self.word[self.position - len + c.len_utf16()..]
                            .chars()
                            .next()
                            .expect("Since remaining > 0, exist the other suffix to expand");

                        self.active = Active::Edge {
                            node: link,
                            edge: next,
                            len: len - c.len_utf8(),
                        };

                        if let Some(created_node) = self.step(c, data) {
                            link = created_node;
                        }
                    }

                    data.get_node_mut(node).link = Some(link);

                    Some(created_node)
                }
            }

            Active::Node { node } => {
                // Insert the character into the internal/leaf node
                if data.get_edge(node, c).is_some() {
                    // Case C
                    self.active = Active::Edge {
                        node,
                        edge: c,
                        len: c.len_utf8(),
                    };

                    self.remaining += 1;

                    None
                } else {
                    // Case B
                    let node_index = self.create_node_leaf(data);
                    data.insert_edge(node, c, node_index);

                    Some(node_index)
                }
            }
        }
    }

    pub(super) fn create_node_leaf(&self, data: &mut SuffixTreeData) -> NodeIndex {
        data.insert_node(Node {
            start: self.position,
            end: self.word.len(),
            link: None,
        })
    }
    pub(super) fn create_node_split_edge(&self, data: &mut SuffixTreeData) -> NodeIndex {
        let Active::Edge {
            node,
            edge: edge_char,
            len: offset,
        } = self.active
        else {
            unreachable!("Cannot split if self.current is not an edge");
        };

        let Some((edge_index, edge)) = data.get_edge_mut(node, edge_char) else {
            unreachable!("Cannot split if edge is not found");
        };

        let left = self.word[edge.start + offset..].chars().next().unwrap();
        let right = self.word[self.position..]
            .chars()
            .next()
            .unwrap_or(TERMINAL);

        debug_assert!(offset < edge.end - edge.start);
        debug_assert!(self.word[edge.start + offset..].chars().next() == Some(left));

        debug_assert!(left != right);

        let mid = Node {
            start: edge.start,
            end: edge.start + offset,
            link: None,
        };

        let right_node = Node {
            start: self.position,
            end: self.word.len(),
            link: None,
        };

        // left node the remaining of old edge
        *edge = Node {
            start: edge.start + offset,
            end: edge.end,
            link: edge.link,
        };

        let mid_index = NodeIndex::new(data.nodes.len());
        data.nodes.push(mid);

        let right_node_index = NodeIndex::new(data.nodes.len());
        data.nodes.push(right_node);

        // replace the pointer to the old edge with the new mid node
        data.edges.insert((node, edge_char), mid_index);

        data.edges.insert((mid_index, left), edge_index);
        data.edges.insert((mid_index, right), right_node_index);

        mid_index
    }
}
