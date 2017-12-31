use graph::*;
use geometry::*;

pub fn single_centered_node(size: &Size) -> Graph<Point> {
    let mut graph = Graph::new();
    graph.add_node(size.center());
    graph
}
