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
        match rng.gen_range(0, 8) {
            0 => splitters.push((group_i, edge_i)),
            1 => dead.push(graph.edge(group_i, edge_i).0),
            _ => { ; }
        }
    });
    for split in splitters {
        let node_i = graph.split_edge_mut(split.0, split.1);
        let new_point = graph.node(node_i).data.clone();
        if rng.gen() {
            let h = rng.gen_range(3.0, 6.0) * if rng.gen() { -1.0 } else { 1.0 };
            let v = rng.gen_range(3.0, 6.0) * if rng.gen() { -1.0 } else { 1.0 };
            let push = Vector::new(h, v, 0.0);
            graph.update_node_data(node_i, new_point + push);
        }
    }
    for node in dead {
        graph.kill_node(node);
    }
    graph
}
