extern crate rand;
use rand::Rng;
use graph::*;
use geometry::*;

#[allow(dead_code)]
pub fn build_centered(size: &Size) -> Graph<Point> {
    let count = rand::thread_rng().gen_range(4, 10);
    let dist_between = 5.0;
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
    let center = size.center();

    for _ in 0..count {
        let x = rand::thread_rng().gen_range(0.0, size.width + 1.0);
        let y = rand::thread_rng().gen_range(0.0, size.height + 1.0);
        let point = Point::new(x, y, 0.0);
        let dist = point.dist(center);
        let circle_size = (size.width / 2.0 - dist) / size.width;
        let r = circle_size * 240.0;
        let group_i = graph.new_group(true);
        for point in make_circle(&point, r) {
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

pub fn lines(count: i32, size: &Size, rot: f32, mut graph: Graph<Point>) -> Graph<Point> {
    let dist_between = size.width / count as f32;

    for i in 0..count {
        let group_i = graph.new_group(false);
        let x = i as f32 * dist_between;
        for point in make_line_at_x(x, size.height, rot) {
            graph.extend(group_i, point);
        }
    }
    graph
}

#[macro_export] macro_rules! build_lines {
    ($a: expr, $b: expr, $c: expr, $d: expr) => { input::graph::lines($a, $b, $c, $d) };
    ($a: expr, $b: expr, $c: expr) => { input::graph::lines($a, $b, $c, graph::Graph::new()) }
}

#[allow(dead_code)]
fn make_line_at_x(x: f32, height: f32, rot: f32) -> Vec<Point> {
    let segments = 40;
    let dist_between = height / segments as f32;

    (0..segments+1).map(|i| {
        Point::new(x + (i as f32) * rot, i as f32 * dist_between, 0.0)
    }).collect()
}
