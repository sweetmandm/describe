extern crate rand;

use geometry::*;
use rand::Rng;

pub fn regular_lines(size: &Size) -> Group {
    let count = rand::thread_rng().gen_range(280, 320);
    let dist_between = size.width / count as f32;

    let paths = (0..count+1).map(|i| {
        let x = i as f32 * dist_between;
        Path {
            points: make_line_at_x(x, size.height),
            closed: false
        }
    }).collect();
    Group { paths }
}

fn make_line_at_x(x: f32, height: f32) -> Vec<Point> {
    let segments = 1300;
    let dist_between = height / segments as f32;

    (0..segments+1).map(|i| {
        Point::new(x, i as f32 * dist_between, 0.0)
    }).collect()
}
