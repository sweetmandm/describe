use geometry::*;
use graph::*;

pub fn build(graph: Graph<Point>, size: &Size) -> String {
    let head = format!(r#"<svg width="{}" height="{}" viewbox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#, size.width, size.height, size.width, size.height);

    let mut body = "".to_string();
    graph.each_edge(&mut|g, e| {
        let edge = &graph.groups[g].edges[e];
        body.push_str(&edge_to_svg(&graph, edge.a, edge.b));
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

fn edge_to_svg(graph: &Graph<Point>, a: NodeIndex, b: NodeIndex) -> String {
    let from = graph.node(a);
    let to = graph.node(b);
    if from.dead || to.dead { return "".to_string() }
    let n = node_to_svg(to);
    let s = format!(r#"<path d="M {} {} L {} {}"
                    style="fill:none;stroke-width:1;stroke:rgb(0,0,0)" />"#, 
                    from.data.x, from.data.y, 
                    to.data.x, to.data.y);
    format!("{}{}", n, s)
}
