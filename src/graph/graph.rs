use graph::node::*;
use graph::edge::*;
use graph::group::*;

pub struct Graph<T: PartialEq> {
    pub nodes: Vec<Node<T>>,
    pub groups: Vec<Group>
}

impl<T: PartialEq> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            nodes: Vec::new(),
            groups: Vec::new()
        }
    }

    pub fn new_group(&mut self, closed: bool) -> GroupIndex {
        let index = self.groups.len();
        let mut group = Group::new();
        group.closed = closed;
        self.groups.push(group);
        index
    }

    pub fn add_node(&mut self, data: T) -> NodeIndex {
        let node_index = self.nodes.len();
        let node = Node::new(data);
        self.nodes.push(node);
        node_index
    }

    pub fn extend(&mut self, group_i: GroupIndex, data: T) -> Option<EdgeIndex> {
        let new_index = self.add_node(data);
        if self.groups[group_i].root == None {
            self.groups[group_i].root = Some(new_index);
            return None;
        }
        let source = match self.groups[group_i].edges.last() {
            Some(e) => e.b,
            None => self.groups[group_i].root.unwrap()
        };
        match self.add_edge(group_i, (source, new_index)) {
            Some(e) => Some(e),
            None => None
        }
    }

    #[allow(dead_code)]
    pub fn kill_node(&mut self, ix: NodeIndex) {
        (&mut self.nodes[ix]).dead = true;
    }

    #[allow(dead_code)]
    pub fn update_node_data(&mut self, ix: NodeIndex, data: T) {
        (&mut self.nodes[ix]).data = data;
    }

    pub fn node(&self, node_index: NodeIndex) -> &Node<T> {
        &self.nodes[node_index]
    }

    pub fn add_edge(&mut self,
                    group_i: GroupIndex,
                    e: (NodeIndex, NodeIndex))
        -> Option<EdgeIndex>
    {
        if self.node(e.0).data == self.node(e.1).data { return None; }
        let group = &mut self.groups[group_i];
        let edge_index = group.edges.len();
        let edge = Edge {
            a: e.0,
            b: e.1
        };
        group.edges.push(edge);
        Some(edge_index)
    }

    pub fn update_edge(&mut self,
                       group_i: GroupIndex,
                       e: EdgeIndex,
                       value: (NodeIndex, NodeIndex))
    {
        if self.node(value.0).data == self.node(value.1).data { return; }
        let group = &mut self.groups[group_i];
        group.edges[e].a = value.0;
        group.edges[e].b = value.1;
    }

    pub fn each_edge<F>(&self, f: &mut F)
    where F: FnMut(GroupIndex, EdgeIndex)
    {
        for (group_i, g) in (&self.groups).iter().enumerate() {
            for e in 0..g.edges.len() {
                f(group_i, e);
            }
        }
    }

    pub fn edge(&self, group_i: GroupIndex, edge_i: EdgeIndex) -> (NodeIndex, NodeIndex) {
        let e = &self.groups[group_i].edges[edge_i];
        (e.a, e.b)
    }
}

