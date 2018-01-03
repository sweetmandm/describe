extern crate rand;
extern crate euclid;

mod geometry;
mod disturb;
mod input;
mod output;
mod graph;
use geometry::*;

fn main() {
    divide();
}

#[allow(dead_code)]
fn divide() {
    let size = Size::new(1024.0, 768.0);
    let mut graph = input::graph::build_centered(&size);
    graph = disturb::divide::run(graph);
    let svg = output::graph_svg::build(graph, &size);

    println!("{}", svg)
}

#[allow(dead_code)]
fn lines() {
    let in_size = Size::new(1024.0, 768.0);
    let out_size = Size::new(2024.0, 1768.0);

    let mut geom = input::lines::build(&in_size);
    geom = disturb::jagged_waves::apply(geom);
    let svg = output::svg::build(geom, &out_size);

    println!("{}", svg)
}

