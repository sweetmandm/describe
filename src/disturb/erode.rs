extern crate rand;
use rand::Rng;
use geometry::*;
use graph::*;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

#[allow(dead_code)]
pub fn run(mut graph: Graph<Point>, size: &Size, iterations: i32) -> Graph<Point> {
    for _ in 0..iterations {
        graph = step(graph, size);
    }
    graph
}

fn step(mut graph: Graph<Point>, size: &Size) -> Graph<Point> {
    let mut dead = Vec::new();

    let x = rand::thread_rng().gen_range(0.0, size.width + 1.0);
    let y = rand::thread_rng().gen_range(0.0, size.height + 1.0);
    let point = Point::new(x, y, 0.0);
    let r = 8.0;

    graph.each_edge(&mut|group_i, edge_i| {
        let (a, b) = graph.edge(group_i, edge_i);
        let candidate_point = graph.node(a).data;
        if candidate_point.dist(point) < r {
            dead.push(a);
        }
    });
    for node in dead {
        graph.kill_node(node);
    }
    graph
}
