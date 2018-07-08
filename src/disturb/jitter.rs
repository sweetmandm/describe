extern crate rand;
use rand::Rng;
use geometry::*;
use graph::*;

pub fn run(graph: Graph<Point>, x_amt: f32, y_amt: f32) -> Graph<Point> {
    graph.jitter(x_amt, y_amt)
}

impl Graph<Point> {
    fn jitter(mut self, x_amt: f32, y_amt: f32) -> Self {
        for node_i in 0..self.nodes.len() {
            let x: f32 = rand::thread_rng().gen_range(-x_amt, x_amt);
            let y: f32 = rand::thread_rng().gen_range(-y_amt, y_amt);
            let pos = self.nodes[node_i].data;
            let new_pos = Point::new(pos.x + x, pos.y + y, pos.z);
            self.update_node_data(node_i, new_pos);
        }
        self
    }
}
