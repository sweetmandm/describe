extern crate rand;
extern crate euclid;

mod geometry;
mod disturb;
mod input;
mod output;
use geometry::*;

fn main() {
    let in_size = Size { width: 1024.0, height: 768.0 };
    let out_size = Size { width: 2024.0, height: 1768.0 };

    let geom = input::regular_lines(&in_size);
    let result = disturb::jagged_waves(geom);
    let svg = output::group_to_svg(result, &out_size);

    println!("{}", svg)
}
