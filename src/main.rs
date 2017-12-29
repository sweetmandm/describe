extern crate rand;
extern crate euclid;

mod geometry;
mod disturb;
mod input;
use geometry::*;

fn main() {
    let size = Size { width: 1024.0, height: 768.0 };
    let paths = input::regular_lines(&size);
    let result = disturb::jagged_waves(paths);
    let svg = lines_to_svg(result, &size);
    println!("{}", svg)
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
