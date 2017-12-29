extern crate euclid;

use euclid::*;

pub type Point = Point3D<f32>;
pub type Line = Vec<Point>;
pub type Vector = Vector3D<f32>;

pub struct Size { 
    pub width: f32,
    pub height: f32
}
