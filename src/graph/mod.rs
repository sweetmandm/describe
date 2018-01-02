pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
    pub edges: Vec<Edge>
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
            edges: Vec::new()
        }
    }

    pub fn add_node(&mut self, data: T) -> NodeIndex {
        let node_index = self.nodes.len();
        let node = Node::new(data);
        self.nodes.push(node);
        node_index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node = &mut self.nodes[source];
        let edge = Edge {
            target: target,
            next: node.first_edge
        };
        self.edges.push(edge);
        node.first_edge = Some(edge_index);
    }

    #[allow(dead_code)]
    pub fn kill_node(&mut self, ix: NodeIndex) {
        (&mut self.nodes[ix]).dead = true;
    }

    pub fn update_node_data(&mut self, ix: NodeIndex, data: T) {
        (&mut self.nodes[ix]).data = data;
    }

    pub fn node(&self, node_index: NodeIndex) -> &Node<T> {
        &self.nodes[node_index]
    }

    pub fn edge(&self, edge_index: EdgeIndex) -> &Edge {
        &self.edges[edge_index]
    }

    pub fn successors(&self, source: NodeIndex) -> Successors<T> {
        let first_edge = self.nodes[source].first_edge;
        Successors { graph: self, current_edge_index: first_edge }
    }
}

pub struct Successors<'graph, T: 'graph> {
    graph: &'graph Graph<T>,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph, T> Iterator for Successors<'graph, T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_i) => {
                let edge = &self.graph.edge(edge_i);
                self.current_edge_index = edge.next;
                Some(edge_i)
            }
        }
    }
}

