extern crate euclid;
use euclid::Size2D;
use geometry::Point;

pub type Size = Size2D<f32>;

pub trait Center {
    fn center(&self) -> Point;
}

impl Center for Size {
    fn center(&self) -> Point {
        return Point::new(self.width / 2.0, self.height / 2.0, 0.0);
    }
}
