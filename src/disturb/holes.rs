extern crate rand;
use rand::Rng;
use geometry::*;
use graph::*;

pub fn apply(graph: Graph<Point>) -> Graph<Point> {
    graph.moo()
}

impl Graph<Point> {
    fn moo(mut self) -> Self {
        for node_i in 0..self.nodes.len() {
            let x: f32 = rand::thread_rng().gen_range(-0.5, 0.5);
            let y: f32 = rand::thread_rng().gen_range(-2.0, 2.0);
            let pos = self.nodes[node_i].data;
            let mut new_pos = Point::new(pos.x + x, pos.y + y, pos.z);
            self.update_node_data(node_i, new_pos);
        }
        self
    }
}

fn sign(val: f32) -> f32 {
    if val < 0.0 { -1.0 } else { 1.0 }
}
