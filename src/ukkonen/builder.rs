use crate::ukkonen::data::{Node, NodeIndex, SuffixTreeData};
use chek::debug_unreachable_unchecked;
use debug_print::debug_println;

pub(super) struct SuffixTreeBuilder<'a> {
    pub(super) word: &'a str,
    pub(super) position: usize,
    pub(super) remaining: usize,

    pub(super) last_created_node: Option<NodeIndex>,

    pub(super) active: Active,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Active {
    node: NodeIndex,
    edge: Option<char>,
    len: usize,
}

pub const TERMINAL: char = '$';

impl<'a> SuffixTreeBuilder<'a> {
    pub fn build_tree(word: &'a str) -> super::SuffixTree<'a> {
        let mut data = SuffixTreeData::new_with_root();
        let mut this = Self {
            word,
            active: Active {
                node: NodeIndex::root(),
                edge: None,
                len: 0,
            },

            last_created_node: None,
            remaining: 0,
            position: 0,
        };

        for c in word.chars().chain(std::iter::once(TERMINAL)) {
            debug_println!("\n Adding '{c}':");
            this.last_created_node = None;

            loop {
                let is_implict_add = this.add_next_suffix(&mut data);

                // After adding with case C/D
                if is_implict_add {
                    this.remaining += c.len_utf8();
                    break;
                }

                if this.remaining == 0 {
                    break;
                }

                // After adding with case B/E
                // must proceed to find for the next active point
                let suffix = &this.word[this.position - this.remaining..this.position];

                let remaining_char = suffix
                    .chars()
                    .next()
                    .expect("Since remaining > 0, exist the other suffix to expand");

                this.remaining -= remaining_char.len_utf8();
                this.active = this.find_next_active(&data);
            }

            this.position += c.len_utf8();
        }

        super::SuffixTree { data, word }
    }

    fn add_next_suffix(&mut self, data: &mut SuffixTreeData) -> bool {
        let c = self.word[self.position..]
            .chars()
            .next()
            .unwrap_or(TERMINAL);

        let Active { node, edge, len } = self.active;

        if let Some(edge) = edge {
            // If panic, this represents a Active::Edge that still is not created
            // what means the scaning point for a invalid state, so need debug to fix.
            let (edge_index, edgen) = data
                .get_edge(node, edge)
                .expect("This represents a Active::Edge that still is not created");

            let other = self.word[edgen.start + len..self.position]
                .chars()
                .next()
                .unwrap_or(TERMINAL);

            if other == c {
                // Case D
                debug_println!("Implicity add '{c}' (Case D)");
                self.active = if len + other.len_utf8() == edgen.end - edgen.start {
                    Active {
                        node: edge_index,
                        edge: None,
                        len: 0,
                    }
                } else {
                    Active {
                        node,
                        edge: Some(edge),
                        len: len + other.len_utf8(),
                    }
                };

                true
            } else {
                // Case E

                let edge_label = &self.word[edgen.start..edgen.end];
                let created_node = self.create_node_split_edge(data);
                debug_println!(
                    "Spliting edge ({}, {}) at ({}) and creating node {}",
                    node.get(),
                    edge_label,
                    len,
                    created_node.get()
                );

                if let Some(last_created_node) = self.last_created_node {
                    debug_println!(
                        "Creating link with: {} -> {}",
                        last_created_node.get(),
                        created_node.get()
                    );

                    data.insert_link(last_created_node, created_node);
                }
                self.last_created_node = Some(created_node);

                false
            }
        } else {
            // Insert the character into the internal/leaf node
            if data.get_edge(node, c).is_some() {
                // Case C
                debug_println!("Implicity add '{c}' (Case C)");
                self.active = Active {
                    node,
                    edge: Some(c),
                    len: c.len_utf8(),
                };

                true
            } else {
                // Case B
                let leaf = self.create_node_leaf(data);
                data.insert_edge(node, c, leaf);
                debug_println!(
                    "Creating leaf node ({}) that follows ({}, {})",
                    leaf.get(),
                    node.get(),
                    c
                );

                if let Some(last_created_node) = self.last_created_node {
                    debug_println!(
                        "Creating link with: {} -> {}",
                        last_created_node.get(),
                        node.get()
                    );

                    data.insert_link(last_created_node, node);
                }
                self.last_created_node = Some(node);

                false
            }
        }
    }

    fn find_next_active(&self, data: &SuffixTreeData) -> Active {
        let mut suffix = &self.word[self.position - self.remaining..];

        let mut next_node = data.get_link(self.active.node).unwrap_or(NodeIndex::root());

        let mut next_edge = self.active.edge;
        let mut next_len = if self.remaining > self.active.len {
            self.active.len
        } else {
            next_edge = suffix.chars().next();
            self.remaining
        };

        loop {
            let Some(edge) = next_edge else {
                next_len = 0;
                break;
            };
            let Some((edge_node, edge)) = data.get_edge(next_node, edge) else {
                next_len = 0;
                next_edge = None;
                break;
            };

            if edge.end - edge.start < next_len {
                suffix = &suffix[edge.end - edge.start..];

                next_len -= edge.end - edge.start;
                next_node = edge_node;
                next_edge = suffix.chars().next();
            } else {
                break;
            }
        }

        debug_println!(
            "Next active point: node={}, edge={:?}, len={}",
            next_node.get(),
            next_edge,
            next_len
        );

        Active {
            node: next_node,
            edge: next_edge,
            len: next_len,
        }
    }

    /// Must be called in when active.edge is None (Case B)
    fn create_node_leaf(&self, data: &mut SuffixTreeData) -> NodeIndex {
        data.insert_node(Node {
            start: self.position,
            end: self.word.len(),
        })
    }

    /// Must be called in when active.edge is Some(_) (Case E)
    fn create_node_split_edge(&self, data: &mut SuffixTreeData) -> NodeIndex {
        let Active {
            node,
            edge: Some(edge_char),
            len,
        } = self.active
        else {
            unsafe {
                debug_unreachable_unchecked!(
                    "Since this function only called in Case E, already checked that edge is not None"
                )
            }
        };

        let Some((edge_index, edge)) = data.get_edge_mut(node, edge_char) else {
            unsafe {
                debug_unreachable_unchecked!(
                    "Since this function only called in Case E, already checked that edge exists"
                )
            }
        };

        let left = self.word[edge.start + len..].chars().next().unwrap();
        let right = self.word[self.position..]
            .chars()
            .next()
            .unwrap_or(TERMINAL);

        debug_assert!(len < edge.end - edge.start);
        debug_assert!(self.word[edge.start + len..].chars().next() == Some(left));

        debug_assert!(left != right);

        let mid = Node {
            start: edge.start,
            end: edge.start + len,
        };

        let right_node = Node {
            start: self.position,
            end: self.word.len(),
        };

        // left node the remaining of old edge
        *edge = Node {
            start: edge.start + len,
            end: edge.end,
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
