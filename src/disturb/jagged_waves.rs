extern crate rand;

use geometry::*;
use rand::Rng;

pub fn apply(group: SvgGroup) -> SvgGroup {
    group.jaggify()
         .shift(Vector::new(1036.0, 20.0, 0.0))
         .swap_xy()
         .jaggify()
         .shift(Vector::new(1036.0, 20.0, 0.0))
}

impl SvgGroup {
    fn jaggify(&self) -> SvgGroup {
        let wind_dir = Point::new(0.4, -1.0, 0.0);
        let wave_height = rand::thread_rng().gen_range(1.0, 8.0);
        let time = 800.0;

        self.each_point(&|pos| {
            let x_height = (1.0 * (pos.x + (wind_dir.y * time * 0.001))).sin() + 1.0;
            let y_height = (pos.y / 1.2 + (-wind_dir.y * time * 0.001)).sin();
            let main_wave = x_height + y_height;
            let aug_y_height = (pos.y / 2.0 + (-wind_dir.y * time * 0.1)).sin();
            let aug_x_height = (pos.x / 2.0 + (-wind_dir.x * time * 0.1)).sin();
            let mut aug_wave = aug_x_height + aug_y_height;
            let triangle = (aug_wave + 0.5).abs() * 2.0;
            aug_wave *= (time * 0.01).sin();
            let wave = wave_height * ((main_wave + aug_wave + triangle) / 2.0);
            let mut new_pos = Point::new(pos.x, pos.y, pos.z);
            new_pos.z = -(pos.x*pos.x / 40.0).abs();
            new_pos.x *= new_pos.z / new_pos.x / 20.0 * sign(pos.x);
            let center = Vector::new(pos.x, pos.y, pos.z);
            let unit = if center.length() > 0.0 {
                center / center.length()
            } else {
                center
            };
            new_pos.x += unit.x * wave * 0.3;
            new_pos.y += unit.y * (wave * 0.2);
            new_pos
        })
    }

    pub fn shift(&self, amount: Vector) -> SvgGroup {
        self.each_point(&|pos| {
            *pos + amount
        })
    }

    pub fn swap_xy(&self) -> SvgGroup {
        self.each_point(&Point::yx)
    }
}

fn sign(val: f32) -> f32 {
    if val < 0.0 { -1.0 } else { 1.0 }
}
