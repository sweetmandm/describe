use geometry::*;
use graph::*;

pub fn build(graph: Graph<Point>, size: &Size) -> String {
    let head = format!(r#"<svg width="{}" height="{}" viewbox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#, size.width, size.height, size.width, size.height);

    let body = graph.nodes.iter().fold("".to_string(), |val, node| {
        let edge = node.first_edge.map(|e| edge_to_svg(node, graph.edge_target(e)));
        [
            val, 
            "\n".to_string(), 
            node_to_svg(node),
            edge.unwrap_or("".to_string())
        ].concat()
    });

    let foot = "</svg>";

    [&head, &body, foot].join("\n")
}

fn node_to_svg(node: &Node<Point>) -> String {
    if node.dead { return "".to_string(); }
    format!(r#"<circle cx="{}" cy="{}" r="1.0" style="fill:none;stroke-width:1;stroke:rgb(0,0,0)" />"#, node.data.x, node.data.y)
}

fn edge_to_svg(node: &Node<Point>, target: &Node<Point>) -> String {
    if node.dead || target.dead { return "".to_string(); }
    format!(r#"<path d="M {} {} L {} {}"
            style="fill:none;stroke-width:1;stroke:rgb(0,0,0)" />"#, 
            node.data.x, node.data.y, 
            target.data.x, target.data.y)
}
