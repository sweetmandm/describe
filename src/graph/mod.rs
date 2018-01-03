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
    pub first_edge: Option<EdgeIndex>,
    pub dead: bool
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node { data, first_edge: None, dead: false }
    }
}

pub type EdgeIndex = usize;

pub struct Edge {
    pub target: NodeIndex,
    pub next: Option<EdgeIndex>
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
        if self.groups[group_i].root == None { return None; }
        let new_index = self.add_node(data);
        let source = match self.groups[group_i].edges.last() {
            Some(e) => e.target,
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
        let node = &mut self.nodes[source];
        let edge = Edge {
            target: target,
            next: node.first_edge
        };
        group.edges.push(edge);
        node.first_edge = Some(edge_index);
        edge_index
    }

    pub fn successors<'a>(&'a self, group: &'a Group, source: NodeIndex) -> Successors {
        let current_edge_index = self.nodes[source].first_edge;
        Successors { group, current_edge_index }
    }

    pub fn traverse<F>(&self,
                      source_i: Option<NodeIndex>,
                      target_i: NodeIndex,
                      seen: &mut Vec<NodeIndex>,
                      f: &mut F)
        where F: FnMut(Option<NodeIndex>, NodeIndex)
        {
            if seen.contains(&target_i) { return; }
            f(source_i, target_i);
            seen.push(target_i);
            for g in self.groups.iter() {
                for e in self.successors(&g, target_i) {
                    let next_target = g.edge(e).target;
                    self.traverse(Some(target_i), next_target, seen, f);
                }
            }
        }

}

impl Group {
    pub fn new() -> Group {
        Group { root: None, edges: Vec::new(), closed: false }
    }

    pub fn edge(&self, edge_index: EdgeIndex) -> &Edge {
        &self.edges[edge_index]
    }
}

pub struct Successors<'graph> {
    group: &'graph Group,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_i) => {
                let edge = &self.group.edge(edge_i);
                self.current_edge_index = edge.next;
                Some(edge_i)
            }
        }
    }
}

