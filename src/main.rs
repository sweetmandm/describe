extern crate rand;
extern crate euclid;

mod geometry;
mod disturb;
mod input;
mod output;
use geometry::*;

fn main() {
    let size = Size { width: 1024.0, height: 768.0 };
    let paths = input::regular_lines(&size);
    let result = disturb::jagged_waves(paths);
    let svg = output::lines_to_svg(result, &size);
    println!("{}", svg)
}
