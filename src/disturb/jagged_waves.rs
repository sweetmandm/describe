extern crate rand;
use rand::Rng;
use geometry::*;
use graph::*;

pub fn apply(graph: Graph<Point>) -> Graph<Point> {
    graph.jaggify()
         .shift(Vector::new(1036.0, 20.0, 0.0))
         .swap_xy()
         .jaggify()
         .shift(Vector::new(1036.0, 20.0, 0.0))
}

impl Graph<Point> {
    fn jaggify(mut self) -> Self {
        let wind_dir = Point::new(0.4, -1.0, 0.0);
        let wave_height = rand::thread_rng().gen_range(150.0, 380.0);
        let time = 800.0;

        for node_i in 0..self.nodes.len() {
            let pos = self.nodes[node_i].data;
            let x_height = (1.0 * (pos.x + (wind_dir.y * time * 0.001))).sin() + 1.0;
            let y_height = (pos.y / 1.2 + (-wind_dir.y * time * 0.001)).sin();
            let main_wave = x_height + y_height;
            let wave = wave_height * ((main_wave) / 2.0);
            let mut new_pos = Point::new(pos.x, pos.y, pos.z);
            new_pos.z = -(pos.x*pos.x / 40.0).abs();
            new_pos.x *= new_pos.z / new_pos.x / 20.0 * sign(pos.x);
            let center = Vector::new(pos.x, pos.y, pos.z);
            let unit = if center.length() > 0.0 {
                center / center.length()
            } else {
                center
            };
            new_pos.x += unit.x * wave * 0.3;
            new_pos.y += unit.y * (wave * 0.2);
            self.update_node_data(node_i, new_pos);
        }
        self
    }
}

fn sign(val: f32) -> f32 {
    if val < 0.0 { -1.0 } else { 1.0 }
}
