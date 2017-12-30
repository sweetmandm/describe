use path::*;
use point::*;

pub struct Group {
    pub paths: Vec<Path>
}

impl Group {
    pub fn each_point(&self, func: &Fn(&Point) -> Point) -> Self {
        Group {
            paths: self.paths.iter().map(|path| {
                Path {
                    points: path.points.iter().map(&func).collect(),
                    closed: path.closed
                }
            }).collect()
        }
    }
}
