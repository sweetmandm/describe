use geometry::*;
use graph::*;

pub fn build(graph: Graph<Point>, size: &Size) -> String {
    let head = format!(r#"<svg width="{}" height="{}" viewbox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#, size.width, size.height, size.width, size.height);

    let mut body = "".to_string();
    let mut seen = Vec::new();
    graph.traverse(None, 0, &mut seen, &mut|from_i_opt, to_i| {
        if let Some(from_i) = from_i_opt {
            body.push_str(&edge_to_svg(&graph, from_i, to_i));
        }
    });

    let foot = "</svg>";

    [&head, &body, foot].join("\n")
}

fn node_to_svg(node: &Node<Point>) -> String {
    if node.dead { return "".to_string(); }
    format!(r#"<circle cx="{}" cy="{}" r="2.0"
            style="fill:none;stroke-width:1;stroke:rgb(0,0,0)" />"#,
            node.data.x, node.data.y)
}

fn edge_to_svg(graph: &Graph<Point>, from_i: NodeIndex, to_i: NodeIndex) -> String {
    let from = graph.node(from_i);
    let to = graph.node(to_i);
    let n = node_to_svg(from);
    let s = format!(r#"<path d="M {} {} L {} {}"
                    style="fill:none;stroke-width:1;stroke:rgb(0,0,0)" />"#, 
                    from.data.x, from.data.y, 
                    to.data.x, to.data.y);
    format!("{}{}", n, s)
}
