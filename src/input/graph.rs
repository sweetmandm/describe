extern crate rand;
use rand::Rng;
use graph::*;
use geometry::*;

#[allow(dead_code)]
pub fn build_centered(size: &Size) -> Graph<Point> {
    let count = rand::thread_rng().gen_range(10, 30);
    let dist_between = size.width / count as f32;
    let center = Point::new(size.width / 2.0, size.height / 2.0, 0.0);
    let mut graph = Graph::new();

    for i in 0..count+1 {
        let r = i as f32 * dist_between;
        let group_i = graph.new_group(true);
        for point in make_circle(&center, size.height / 8.0 + r) {
            graph.extend(group_i, point);
        }
    }
    graph
}

#[allow(dead_code)]
pub fn build_random(size: &Size) -> Graph<Point> {
    let count = rand::thread_rng().gen_range(8, 30);
    let mut graph = Graph::new();

    for _ in 0..count+1 {
        let x = rand::thread_rng().gen_range(0.0, size.width);
        let y = rand::thread_rng().gen_range(0.0, size.height);
        let r = rand::thread_rng().gen_range(size.width / 18.0, size.width / 10.0);
        let center = Point::new(x, y, 0.0);
        let group_i = graph.new_group(true);
        for point in make_circle(&center, size.height / 8.0 + r) {
            graph.extend(group_i, point);
        }
    }
    graph
}

#[allow(dead_code)]
fn make_circle(center: &Point, radius: f32) -> Vec<Point> {
    let segments = 100;
    let dist_between = ::std::f32::consts::PI * 2.0 / segments as f32;

    (0..segments+1).map(|i| {
        let theta = dist_between * i as f32;
        let x = center.x + radius * theta.cos();
        let y = center.y + radius * theta.sin();
        Point::new(x, y, 0.0)
    }).collect()
}
