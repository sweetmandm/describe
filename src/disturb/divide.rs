extern crate rand;
use rand::Rng;
use geometry::*;
use graph::*;

#[allow(dead_code)]
pub fn run(mut graph: Graph<Point>, size: &Size) -> Graph<Point> {
    for _ in 0..20 {
        graph = step(graph, size);
    }
    graph
}

fn step(mut graph: Graph<Point>, size: &Size) -> Graph<Point> {
    let mut rng = rand::thread_rng();
    let mut splitters = Vec::new();
    let mut dead = Vec::new();
    graph.each_edge(&mut|group_i, edge_i| {
        match rng.gen_range(1, 8) {
            7 => splitters.push((group_i, edge_i)),
            6 => dead.push(graph.edge(group_i, edge_i).0),
            _ => { ; }
        }
    });
    for split in splitters {
        let node_i = graph.split_edge_mut(split.0, split.1);
        let new_point = graph.node(node_i).data.clone();
        let center = size.center();
        let dx = new_point.x - center.x;
        let dy = new_point.y - center.y;
        let a = dy.atan2(dx);
        let m = rng.gen_range(3.0, 8.0);
        let push = Vector::from_angle_magnitude(a, m);
        graph.update_node_data(node_i, new_point + push);
    }
    for node in dead {
        graph.kill_node(node);
    }
    graph
}
