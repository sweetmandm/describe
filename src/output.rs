use geometry::*;

pub fn lines_to_svg(paths: Vec<Line>, size: &Size) -> String {
    let head = format!(r#"<svg width="{}" height="{}" viewbox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#, size.width, size.height, size.width, size.height);

    let body = paths.iter().fold("".to_string(), |val, line| {
        [val, "\n".to_string(), line_to_path(line)].concat()
    });

    let foot = "</svg>";

    [&head, &body, foot].join("\n")
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
