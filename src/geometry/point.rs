extern crate euclid;
use euclid::Point3D;

pub type Point = Point3D<f32>;

pub trait SwapElements {
    fn yx(&self) -> Self;
}

impl SwapElements for Point {
    fn yx(&self) -> Self {
        Point::new(self.y, self.x, self.z)
    }
}
