extern crate rand;
extern crate euclid;

mod geometry;
mod disturb;
use geometry::*;
use rand::Rng;

fn main() {
    let count = rand::thread_rng().gen_range(280, 320);
    let size = Size { width: 1024.0, height: 768.0 };
    let paths = make_lines(count, &size);
    let result = disturb::jagged_waves(paths);
    let svg = lines_to_svg(result, &size);
    println!("{}", svg)
}

fn make_lines(count: i32, size: &Size) -> Vec<Line> {
    let dist_between = size.width / count as f32;

    (0..count+1).map(|i| {
        let x = i as f32 * dist_between;
        make_line_at_x(x, size.height)
    }).collect()
}

fn make_line_at_x(x: f32, height: f32) -> Line {
    let segments = 30;
    let dist_between = height / segments as f32;

    (0..segments+1).map(|i| {
        Point::new(x, i as f32 * dist_between, 0.0)
    }).collect()
}

fn lines_to_svg(paths: Vec<Line>, size: &Size) -> String {
    let head = format!(r#"<svg width="{}" height="{}" viewbox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#, size.width, size.height, size.width, size.height);

    let body = paths.iter().fold("".to_string(), |val, line| {
        [val, line_to_path(line)].concat()
    });

    let foot = r#"</svg>"#;

    [&head, &body, foot].concat()
}

fn line_to_path(line: &Line) -> String {
    if line.len() < 1 { return "".to_string(); }

    let first = path_op('M', &line[0]);

    let rest = line[1..line.len()].iter().fold("".to_string(), |pts, pt| {
        [pts, path_op('L', pt)].concat()
    });

    format!(r#"<path d="{}{}" style="fill:none;stroke-width:1;stroke:rgb(0,0,0)" />"#, first, rest)
}

fn path_op(op: char, point: &Point) -> String {
    format!("{} {} {} ", op, point.x, point.y)
}
