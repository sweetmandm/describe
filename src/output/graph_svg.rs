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

    let mut body = "".to_string();
    for node in &graph.nodes {
        body.push_str(&node_to_svg(&node, &style));
    }
    graph.each_edge(&mut|g, e| {
        let edge = &graph.groups[g].edges[e];
        let a = graph.node(edge.a);
        let b = graph.node(edge.b);
        body.push_str(&edge_to_svg(a, b, &style));
    });

    let foot = "</svg>";

    [&head, &body, foot].join("\n")
}

fn node_to_svg(node: &Node<Point>, style: &Style) -> String {
    if let Some(node_style) = style.nodes {
        if node.dead { return "".to_string(); }
        format!(r#"<circle cx="{}" cy="{}" r="2.0" style="{}" />"#,
                node.data.x, node.data.y, node_style)
    } else {
        "".to_string()
    }
}

fn edge_to_svg(a: &Node<Point>, b: &Node<Point>, style: &Style) -> String {
    if let Some(edge_style) = style.edges {
        if a.dead || b.dead { return "".to_string(); }
        format!(r#"<path d="M {} {} L {} {}" style="{}" />"#,
                a.data.x, a.data.y,
                b.data.x, b.data.y,
                edge_style)
    } else {
        "".to_string()
    }
}
