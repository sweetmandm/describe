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
    let line_count = 30;
    let mut graph = input::graph::lines(line_count, &size);
    graph = disturb::jagged_waves::apply(graph);

    graph = disturb::divide::run(graph, 30);

    let svg = output::graph_svg::build(graph,
                                       &size,
                                       output::graph_svg::Style::edges());

    println!("{}", svg)
}

