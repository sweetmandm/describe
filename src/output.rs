use geometry::*;

pub fn group_to_svg(group: Group, size: &Size) -> String {
    let head = format!(r#"<svg width="{}" height="{}" viewbox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#, size.width, size.height, size.width, size.height);

    let body = group.paths.iter().fold("".to_string(), |val, path| {
        [val, "\n".to_string(), path_to_svg(path)].concat()
    });

    let foot = "</svg>";

    [&head, &body, foot].join("\n")
}

fn path_to_svg(path: &Path) -> String {
    let points = &path.points;
    if points.len() < 1 { return "".to_string(); }

    let first = path_op('M', &points[0]);

    let mut rest = points[1..points.len()].iter().fold("".to_string(), |pts, pt| {
        [pts, path_op('L', pt)].concat()
    });

    if path.closed { rest.push_str("Z"); }

    format!(r#"<path d="{}{}" style="fill:none;stroke-width:1;stroke:rgb(0,0,0)" />"#, first, rest)
}

fn path_op(op: char, point: &Point) -> String {
    format!("{} {} {} ", op, point.x, point.y)
}
