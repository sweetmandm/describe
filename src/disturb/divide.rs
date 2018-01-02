extern crate rand;
extern crate euclid;
use rand::Rng;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};
use geometry::*;
use graph::*;

#[allow(dead_code)]
pub fn run(mut graph: Graph<Point>) -> Graph<Point> {
    for _ in 0..40 {
        graph = step(graph);
    }
    graph
}

fn step(mut graph: Graph<Point>) -> Graph<Point> {
    let mut new = Vec::new();
    let mut updated = Vec::new();

    for (i, node) in graph.nodes.iter().enumerate() {
        let r = rand::thread_rng().gen_range(0, 6);
        match r {
            5 => {
                let result = split(node.data);
                updated.push((i, result.0));
                new.push((i, result.1));
            }
            _ => { ; }

        }
    }
    for pair in updated {
        graph.update_node_data(pair.0, pair.1);
    }
    for pair in new {
        let new_node_index = graph.add_node(pair.1);
        graph.add_edge(pair.0, new_node_index);
    }
    graph
}

fn split(loc: Point) -> (Point, Point) {
    let mut items = vec!(Weighted { weight: 8, item: (0.0, ::std::f32::consts::PI) },
                         Weighted { weight: 20, item: (0.1, 0.15) });
    let wc = WeightedChoice::new(&mut items);
    let mut rng = rand::thread_rng();
    let r = wc.ind_sample(&mut rng);

    let angle = rand::thread_rng().gen_range(r.0, r.1);
    let magnitude = rand::thread_rng().gen_range(15.0, 40.0);
    let push = vector_from(angle, magnitude);
    (
        loc + push,
        loc - push
    )
}

fn vector_from(angle: f32, mag: f32) -> Vector {
    Vector::new(mag * angle.cos(), mag * angle.sin(), 0.0)
}
