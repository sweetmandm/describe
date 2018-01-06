use graph::node::*;

pub type EdgeIndex = usize;

pub struct Edge {
    pub a: NodeIndex,
    pub b: NodeIndex,
}

