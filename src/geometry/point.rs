extern crate euclid;
use euclid::Point3D;

pub type Point = Point3D<f32>;

pub trait PointOps {
    fn yx(&self) -> Self;
    fn dist(&self, other: Point) -> f32;
}

impl PointOps for Point {
    fn yx(&self) -> Self {
        Point::new(self.y, self.x, self.z)
    }

    fn dist(&self, other: Point) -> f32 {
        ((other.x - self.x).powf(2.0) +
         (other.y - self.y).powf(2.0) +
         (other.z - self.z).powf(2.0)).sqrt()
    }
}
