#![allow(dead_code)]

// primitives
mod geometry;
use geometry::*;
mod graph;
mod input;

// exporter
mod output;

// algorithms & tools
mod disturb;
mod vis;
extern crate rand;
extern crate euclid;

fn main() {
    vis::holes::run();
}
