extern crate rand;
extern crate euclid;

mod geometry;
mod disturb;
mod input;
mod output;
use geometry::*;

fn main() {
    let in_size = Size::new(1024.0, 768.0);
    let out_size = Size::new(2024.0, 1768.0);

    let mut geom = input::lines::build(&in_size);
    geom = disturb::jagged_waves::apply(geom);
    let svg = output::svg::build(geom, &out_size);

    println!("{}", svg)
}
