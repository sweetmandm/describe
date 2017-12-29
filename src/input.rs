extern crate rand;

use geometry::*;
use rand::Rng;

pub fn regular_lines(size: &Size) -> Vec<Line> {
    let count = rand::thread_rng().gen_range(280, 320);
    let dist_between = size.width / count as f32;

    (0..count+1).map(|i| {
        let x = i as f32 * dist_between;
        make_line_at_x(x, size.height)
    }).collect()
}

fn make_line_at_x(x: f32, height: f32) -> Line {
    let segments = 1300;
    let dist_between = height / segments as f32;

    (0..segments+1).map(|i| {
        Point::new(x, i as f32 * dist_between, 0.0)
    }).collect()
}
