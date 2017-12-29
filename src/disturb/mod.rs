use geometry::*;

pub fn jagged_waves(group: Group) -> Group {
    group.jaggify()
         .swap_xy()
         .jaggify()
         .shift(Point::new(960.0, 600.0, 0.0))
}

impl Group {
    pub fn each_point(&self, func: &Fn(&Point) -> Point) -> Self {
        Group {
            paths: self.paths.iter().map(|path| {
                Path {
                    points: path.points.iter().map(|pos| { func(pos) }).collect(),
                    closed: path.closed
                }
            }).collect()
        }
    }

    fn jaggify(&self) -> Group {
        let wind_dir = Point::new(0.4, -1.0, 0.0);
        let wave_height = 10.0;
        let time = 800.0;

        self.each_point(&|pos| {
            let x_height = (1.0 * (pos.x + (wind_dir.y * time * 0.001))).sin() + 1.0;
            let y_height = (pos.y / 1.2 + (-wind_dir.y * time * 0.001)).sin();
            let main_wave = x_height + y_height;
            let aug_y_height = (pos.y / 2.0 + (-wind_dir.y * time * 0.001)).sin();
            let aug_x_height = (pos.x / 2.0 + (-wind_dir.x * time * 0.001)).sin();
            let mut aug_wave = aug_x_height + aug_y_height;
            let triangle = (aug_wave + 0.5).abs() * 2.0;
            aug_wave *= (time * 0.001).sin();
            let wave = wave_height * ((main_wave + aug_wave + triangle) / 2.0);
            let mut new_pos = Point::new(pos.x, pos.y, pos.z);
            new_pos.z = -(pos.x*pos.x / 40.0).abs();
            new_pos.x += new_pos.z * sign(pos.x);
            let center = Vector::new(pos.x, pos.y, pos.z);
            let unit = if center.length() > 0.0 {
                center / center.length()
            } else {
                center
            };
            new_pos.x += unit.x * wave * 0.3 + 1036.0;
            new_pos.y += unit.y * wave + 20.0;
            new_pos
        })
    }

    pub fn shift(&self, amount: Point) -> Group {
        self.each_point(&|pos| {
            Point::new(pos.x + amount.x, pos.y + amount.y, pos.z)
        })
    }

    pub fn swap_xy(&self) -> Group {
        self.each_point(&|pos| {
            Point::new(pos.y, pos.x, pos.z)
        })
    }
}

fn sign(val: f32) -> f32 {
    if val < 0.0 { -1.0 } else { 1.0 }
}
