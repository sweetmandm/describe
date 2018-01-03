use path::*;
use point::*;

pub struct SvgGroup {
    pub paths: Vec<Path>
}

impl SvgGroup {
    pub fn each_point(&self, func: &Fn(&Point) -> Point) -> Self {
        SvgGroup {
            paths: self.paths.iter().map(|path| {
                Path {
                    points: path.points.iter().map(&func).collect(),
                    closed: path.closed
                }
            }).collect()
        }
    }
}
