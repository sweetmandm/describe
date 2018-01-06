use graph::node::*;
use graph::edge::*;

pub type GroupIndex = usize;

pub struct Group {
    pub root: Option<NodeIndex>,
    pub edges: Vec<Edge>,
    pub closed: bool
}

impl Group {
    pub fn new() -> Group {
        Group { root: None, edges: Vec::new(), closed: false }
    }
}

