extern crate rand;

use geometry::*;
use rand::Rng;

#[allow(dead_code)]
pub fn build_centered(size: &Size) -> Group {
    let count = rand::thread_rng().gen_range(20, 90);
    let dist_between = size.width / count as f32;
    let center = Point::new(size.width / 2.0, size.height / 2.0, 0.0);

    let paths = (0..count+1).map(|i| {
        let r = i as f32 * dist_between;
        Path {
            points: make_circle(&center, size.height / 8.0 + r),
            closed: true
        }
    }).collect();
    Group { paths }
}

#[allow(dead_code)]
pub fn build_random(size: &Size) -> Group {
    let count = rand::thread_rng().gen_range(8, 30);

    let paths = (0..count+1).map(|_| {
        let x = rand::thread_rng().gen_range(0.0, size.width);
        let y = rand::thread_rng().gen_range(0.0, size.height);
        let r = rand::thread_rng().gen_range(size.width / 18.0, size.width / 10.0);
        let center = Point::new(x, y, 0.0);
        Path {
            points: make_circle(&center, r),
            closed: true
        }
    }).collect();
    Group { paths }
}

#[allow(dead_code)]
fn make_circle(center: &Point, radius: f32) -> Vec<Point> {
    let segments = 1300;
    let dist_between = ::std::f32::consts::PI * 2.0 / segments as f32;

    (0..segments+1).map(|i| {
        let theta = dist_between * i as f32;
        let x = center.x + radius * theta.cos();
        let y = center.y + radius * theta.sin();
        Point::new(x, y, 0.0)
    }).collect()
}
