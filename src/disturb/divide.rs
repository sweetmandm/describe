extern crate rand;
use rand::Rng;
use geometry::*;
use graph::*;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

#[allow(dead_code)]
pub fn run(mut graph: Graph<Point>, iterations: i32) -> Graph<Point> {
    for _ in 0..iterations {
        graph = step(graph);
    }
    graph
}

fn step(mut graph: Graph<Point>) -> Graph<Point> {
    let mut rng = rand::thread_rng();
    let mut splitters = Vec::new();
    let mut dead = Vec::new();
    let mut items = vec!(Weighted { weight: 8, item: "split" },
                         Weighted { weight: 2, item: "kill" },
                         Weighted { weight: 44, item: "noop" });
    let wc = WeightedChoice::new(&mut items);

    graph.each_edge(&mut|group_i, edge_i| {
        match wc.ind_sample(&mut rng) {
            "split" => splitters.push((group_i, edge_i)),
            "kill" => dead.push(graph.edge(group_i, edge_i).0),
            _ => { ; }
        }
    });
    for split in splitters {
        let node_i = graph.split_edge_mut(split.0, split.1);
        let new_point = graph.node(node_i).data.clone();
        if rng.gen_range(0.0, 1024.0) > new_point.x {
            let contrib = (1024.0 - new_point.x.abs()) / 1024.0;
            let h = rng.gen_range(10.0, 15.0) * contrib * if rng.gen() { -1.0 } else { 1.0 };
            let v = rng.gen_range(10.0, 15.0) * contrib * if rng.gen() { -1.0 } else { 1.0 };
            let push = Vector::new(h, v, 0.0);
            graph.update_node_data(node_i, new_point + push);
        }
    }
    for node in dead {
        graph.kill_node(node);
    }
    graph
}
