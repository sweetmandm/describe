pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
    pub groups: Vec<Group>
}

pub type GroupIndex = usize;

pub struct Group {
    pub root: Option<NodeIndex>,
    pub edges: Vec<Edge>,
    pub closed: bool
}

pub type NodeIndex = usize;

pub struct Node<T> {
    pub data: T,
    pub dead: bool
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node { data, dead: false }
    }
}

pub type EdgeIndex = usize;

pub struct Edge {
    pub a: NodeIndex,
    pub b: NodeIndex,
}

impl<T> Graph<T> {
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
        Some(self.add_edge(group_i, source, new_index))
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
                    source: NodeIndex,
                    target: NodeIndex)
        -> EdgeIndex
    {
        let group = &mut self.groups[group_i];
        let edge_index = group.edges.len();
        let edge = Edge {
            a: source,
            b: target
        };
        group.edges.push(edge);
        edge_index
    }

    pub fn each_edge<F>(&self, f: &mut F)
    where F: FnMut(NodeIndex, NodeIndex)
    {
        for g in &self.groups {
            for e in &g.edges {
                f(e.a, e.b);
            }
        }
    }
}

impl Group {
    pub fn new() -> Group {
        Group { root: None, edges: Vec::new(), closed: false }
    }
}

