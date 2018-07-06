extern crate rand;
extern crate euclid;

mod geometry;
mod disturb;
#[macro_use] mod input;
mod output;
mod graph;
use geometry::*;

fn main() {
    divide();
}

#[allow(dead_code)]
fn divide() {
    let size = Size::new(1024.0, 768.0);

    let mut graph = build_lines!(300, &size, 0.0);

    graph = disturb::erode::run(graph, &size, 8, (80.0, 100.0));
    graph = disturb::erode::run(graph, &size, 80, (8.0, 10.0));
    graph = disturb::erode::run(graph, &size, 20, (40.0, 60.0));
    graph = disturb::erode::run(graph, &size, 10, (60.0, 100.0));

    graph = disturb::divide::run(graph, 18);
    graph = disturb::erode::run(graph, &size, 10, (60.0, 130.0));
    graph = disturb::divide::run(graph, 2);
    graph = disturb::erode::run(graph, &size, 4, (6.0, 30.0));

    let svg = output::graph_svg::build(graph,
                                       &size,
                                       output::graph_svg::Style::edges());

    println!("{}", svg)
}

