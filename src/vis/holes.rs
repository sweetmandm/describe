use ::disturb;
use ::input;
use ::output;
use ::graph;
use geometry::*;

pub fn run() {
    let size = Size::new(1024.0, 768.0);

    let lines = 300;
    let segments = 40;
    let mut graph = input::graph::build_lines(lines, segments, &size, 0.0, graph::Graph::new());

    graph = disturb::divide::run(graph, 18);
    graph = disturb::erode::run(graph, &size, 10, (60.0, 130.0));
    graph = disturb::divide::run(graph, 1);
    graph = disturb::erode::run(graph, &size, 4, (6.0, 30.0));

    let svg = output::graph_svg::build(graph,
                                       &size,
                                       output::graph_svg::Style::edges());

    println!("{}", svg)
}

