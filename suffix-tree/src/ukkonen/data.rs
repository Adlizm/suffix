use std::collections::{BTreeMap, HashMap};
use std::num::NonZeroUsize;

use crate::word::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct NodeIndex(NonZeroUsize);

impl NodeIndex {
    pub fn root() -> Self {
        Self::new(0)
    }

    /// Creates a new PositiveIsize if the value is <= isize::MAX.
    pub(super) const fn new(val: usize) -> Self {
        if val <= isize::MAX as usize {
            // Safety: val + 1 is guaranteed to be >= 1, so it's never zero.
            // Also, because val <= isize::MAX, val + 1 will never overflow usize.
            Self(unsafe { NonZeroUsize::new_unchecked(val + 1) })
        } else {
            panic!("value must be <= isize::MAX")
        }
    }

    /// Gets the underlying usize value (0..=isize::MAX).
    pub(super) fn get(self) -> usize {
        self.0.get() - 1
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Node {
    pub start: usize,
    pub end: usize,
}

impl Node {
    pub(super) fn root() -> Self {
        Self { start: 0, end: 0 }
    }
}

#[derive(Debug)]
pub struct SuffixTreeData<T> {
    pub(super) nodes: Vec<Node>,
    pub(super) edges: HashMap<NodeIndex, BTreeMap<Symbol<T>, NodeIndex>>,
    pub(super) links: HashMap<NodeIndex, NodeIndex>,
}

impl<T: Copy + Ord> SuffixTreeData<T> {
    pub(super) fn new_with_root() -> Self {
        Self {
            nodes: vec![Node::root()],
            edges: Default::default(),
            links: Default::default(),
        }
    }

    pub(super) fn insert_node(&mut self, node: Node) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(node);

        NodeIndex::new(index)
    }
    pub(super) fn insert_edge(&mut self, from: NodeIndex, edge: Symbol<T>, to: NodeIndex) {
        self.edges.entry(from).or_default().insert(edge, to);
    }
    pub(super) fn insert_link(&mut self, from: NodeIndex, to: NodeIndex) {
        if to == NodeIndex::root() {
            return; // Already default link
        }
        self.links.insert(from, to);
    }

    pub fn get_edges(
        &self,
        node: NodeIndex,
    ) -> Option<impl Iterator<Item = (Symbol<T>, NodeIndex)>> {
        if let Some(edges) = self.edges.get(&node) {
            Some(edges.iter().map(|(key, index)| (*key, *index)))
        } else {
            None
        }
    }

    pub fn get_edge(&self, node: NodeIndex, edge: Symbol<T>) -> Option<(NodeIndex, &Node)> {
        let edge = self.edges.get(&node)?.get(&edge)?;

        self.nodes.get(edge.get()).map(|node| (*edge, node))
    }
    pub fn get_edge_mut(
        &mut self,
        node: NodeIndex,
        edge: Symbol<T>,
    ) -> Option<(NodeIndex, &mut Node)> {
        let edge = self.edges.get(&node)?.get(&edge)?;

        self.nodes.get_mut(edge.get()).map(|node| (*edge, node))
    }

    pub fn get_link(&self, node: NodeIndex) -> Option<NodeIndex> {
        self.links.get(&node).copied()
    }

    pub fn get_node(&self, index: NodeIndex) -> &Node {
        debug_assert!(index.get() < self.nodes.len());

        unsafe { self.nodes.get_unchecked(index.get()) }
    }
    pub fn get_node_mut(&mut self, index: NodeIndex) -> &mut Node {
        debug_assert!(index.get() < self.nodes.len());

        unsafe { self.nodes.get_unchecked_mut(index.get()) }
    }
}
