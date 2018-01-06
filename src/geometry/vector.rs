extern crate euclid;
use euclid::*;

pub type Vector = Vector3D<f32>;

pub trait VectorOps {
    fn from_angle_magnitude(angle: f32, magnitude: f32) -> Self;
}

impl VectorOps for Vector {
    fn from_angle_magnitude(angle: f32, magnitude: f32) -> Self {
        Vector::new(magnitude * angle.cos(), magnitude * angle.sin(), 0.0)
    }
}
