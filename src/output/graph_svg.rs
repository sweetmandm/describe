use geometry::*;
use graph::*;

pub struct Style {
    pub nodes: Option<&'static str>,
    pub edges: Option<&'static str>
}

impl Style {
    pub fn edges() -> Self {
        Style {
            nodes: None,
            edges: Some("fill:none;stroke-width:1;stroke:rgb(0,0,0)")
        }
    }

    pub fn nodes() -> Self {
        Style {
            nodes: Some("fill:none;stroke-width:1;stroke:rgb(0,0,0)"),
            edges: None
        }
    }

    pub fn edges_and_nodes() -> Self {
        Style {
            nodes: Some("fill:none;stroke-width:1;stroke:rgb(0,0,0)"),
            edges: Some("fill:none;stroke-width:1;stroke:rgb(0,0,0)")
        }
    }
}

pub fn build(graph: Graph<Point>, size: &Size, style: Style) -> String {
    let head = format!(r#"<svg width="{}" height="{}" viewbox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#, size.width, size.height, size.width, size.height);

    let mut body: Vec<String> = vec![];
    for node in &graph.nodes {
        if let Some(s) = node_to_svg(&node, &style) {
            body.push(s);
        }
    }
    graph.each_edge(&mut|g, e| {
        let edge = &graph.groups[g].edges[e];
        let a = graph.node(edge.a);
        let b = graph.node(edge.b);
        if let Some(s) = edge_to_svg(a, b, &style) {
            body.push(s);
        }
    });
    let body_str = body.join("\n");

    let foot = "</svg>";

    [&head, &body_str, foot].join("\n")
}

fn node_to_svg(node: &Node<Point>, style: &Style) -> Option<String> {
    if let Some(node_style) = style.nodes {
        let has_nan = [node.data.x, node.data.y].iter().all(|&a| a.is_nan());
        if node.dead || has_nan { return None; }
        Some(format!(r#"<circle cx="{}" cy="{}" r="2.0" style="{}" />"#,
                     node.data.x, node.data.y, node_style))
    } else {
        None
    }
}

fn edge_to_svg(a: &Node<Point>, b: &Node<Point>, style: &Style) -> Option<String> {
    if let Some(edge_style) = style.edges {
        let has_nan = [a.data.x, a.data.y, b.data.x, b.data.y].iter().all(|&a| a.is_nan());
        if has_nan || a.dead || b.dead { return None; }
        Some(format!(r#"<path d="M {} {} L {} {}" style="{}" />"#,
                     a.data.x, a.data.y,
                     b.data.x, b.data.y,
                     edge_style))
    } else {
        None
    }
}
