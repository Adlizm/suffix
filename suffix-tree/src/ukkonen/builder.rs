use chek::debug_unreachable_unchecked;

use crate::ukkonen::data::{Node, NodeIndex, SuffixTreeData};
use crate::word::{Symbol, Word};

pub(super) struct SuffixTreeBuilder<'a, T: ?Sized + Word> {
    pub(super) word: &'a T,
    pub(super) position: usize,
    pub(super) remaining: usize,

    pub(super) last_created_node: Option<NodeIndex>,

    pub(super) active: Active<T::Alphabet>,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Active<T> {
    node: NodeIndex,         // no pai
    edge: Option<Symbol<T>>, // caracter do pai para proximo
    len: usize,
}

impl<'a, T: ?Sized + Word> SuffixTreeBuilder<'a, T> {
    pub fn build_tree(word: &'a T) -> super::SuffixTree<'a, T> {
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

        for c in word
            .symbols()
            .map(Symbol::Char)
            .chain(std::iter::once(Symbol::Terminal))
        {
            this.last_created_node = None;

            loop {
                let is_implict_add = this.add_next_suffix(&mut data);

                // After adding with case C/D
                if is_implict_add {
                    this.remaining += word.size_of_symbol(c);
                    break;
                }

                if this.remaining == 0 {
                    break;
                }

                // After adding with case B/E
                // must proceed to find for the next active point
                let remaining_char = word
                    .try_get(this.position - this.remaining)
                    .expect("Since remaining > 0, exist the other suffix to expand");

                this.remaining -= word.size_of(remaining_char);
                this.active = this.find_next_active(&data);
            }

            // Case A
            this.position += word.size_of_symbol(c);
        }

        super::SuffixTree { data, word }
    }

    fn add_next_suffix(&mut self, data: &mut SuffixTreeData<T::Alphabet>) -> bool {
        let c = self.word.get(self.position);

        let Active { node, edge, len } = self.active;

        if let Some(edge) = edge {
            // If panic, this represents a Active::Edge that still is not created
            // what means the scaning point for a invalid state, so need debug to fix.
            let (edge_index, edgen) = data
                .get_edge(node, edge)
                .expect("This represents a Active::Edge that still is not created");

            let other = self.word.get(edgen.start + len);

            if other == c {
                // Case D
                let other_len = self.word.size_of_symbol(other);
                self.active = if len + other_len == edgen.end - edgen.start {
                    Active {
                        node: edge_index,
                        edge: None,
                        len: 0,
                    }
                } else {
                    Active {
                        node,
                        edge: Some(edge),
                        len: len + other_len,
                    }
                };

                true
            } else {
                // Case E

                let created_node = self.create_node_split_edge(data);
                if let Some(last_created_node) = self.last_created_node {
                    data.insert_link(last_created_node, created_node);
                }
                self.last_created_node = Some(created_node);

                false
            }
        } else {
            // Insert the character into the internal/leaf node
            if data.get_edge(node, c).is_some() {
                // Case C
                self.active = Active {
                    node,
                    edge: Some(c),
                    len: self.word.size_of_symbol(c),
                };

                true
            } else {
                // Case B
                let leaf = self.create_node_leaf(data);
                data.insert_edge(node, c, leaf);

                if let Some(last_created_node) = self.last_created_node {
                    data.insert_link(last_created_node, node);
                }
                self.last_created_node = Some(node);

                false
            }
        }
    }

    fn find_next_active(&self, data: &SuffixTreeData<T::Alphabet>) -> Active<T::Alphabet> {
        let mut suffix = self.position - self.remaining;

        let mut next_node = data.get_link(self.active.node).unwrap_or(NodeIndex::root());

        let mut next_edge = self.active.edge;
        let mut next_len = if self.remaining > self.active.len {
            self.active.len
        } else {
            next_edge = self.word.try_get(suffix).map(Symbol::Char);
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
                suffix += edge.end - edge.start;

                next_len -= edge.end - edge.start;
                next_node = edge_node;
                next_edge = self.word.try_get(suffix).map(Symbol::Char);
            } else {
                break;
            }
        }

        Active {
            node: next_node,
            edge: next_edge,
            len: next_len,
        }
    }

    /// Must be called in when active.edge is None (Case B)
    fn create_node_leaf(&self, data: &mut SuffixTreeData<T::Alphabet>) -> NodeIndex {
        data.insert_node(Node {
            start: self.position,
            end: self.word.size(),
        })
    }

    /// Must be called in when active.edge is Some(_) (Case E)
    fn create_node_split_edge(&self, data: &mut SuffixTreeData<T::Alphabet>) -> NodeIndex {
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

        let left = self.word.get(edge.start + len);
        let right = self.word.get(self.position);

        debug_assert!(len < edge.end - edge.start);
        debug_assert!(self.word.get(edge.start + len) == left);

        debug_assert!(left != right);

        let mid = Node {
            start: edge.start,
            end: edge.start + len,
        };

        let right_node = Node {
            start: self.position,
            end: self.word.size(),
        };

        // left node the remaining of old edge
        *edge = Node {
            start: edge.start + len,
            end: edge.end,
        };

        let mid_index = data.insert_node(mid);

        let right_node_index = data.insert_node(right_node);

        // replace the pointer to the old edge with the new mid node
        data.insert_edge(node, edge_char, mid_index);

        data.insert_edge(mid_index, left, edge_index);
        data.insert_edge(mid_index, right, right_node_index);

        mid_index
    }
}
