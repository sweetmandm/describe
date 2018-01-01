use geometry::*;
use graph::*;

pub fn build(graph: Graph<Point>, size: &Size) -> String {
    let head = format!(r#"<svg width="{}" height="{}" viewbox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#, size.width, size.height, size.width, size.height);

    let body = graph.nodes.iter().fold("".to_string(), |val, node| {
        let edges = edge_to_svg(&graph, "".to_string(), node, node.first_edge, vec!());
        [
            val, 
            "\n".to_string(), 
            node_to_svg(node),
            edges
        ].concat()
    });

    let foot = "</svg>";

    [&head, &body, foot].join("\n")
}

fn node_to_svg(node: &Node<Point>) -> String {
    if node.dead { return "".to_string(); }
    format!(r#"<circle cx="{}" cy="{}" r="1.0" style="fill:none;stroke-width:1;stroke:rgb(0,0,0)" />"#, node.data.x, node.data.y)
}

fn edge_to_svg(graph: &Graph<Point>, 
               acc: String, 
               source: &Node<Point>, 
               edge_i: Option<EdgeIndex>,
               visited: Vec<NodeIndex>) -> String 
{
    if None == edge_i || source.dead { return acc; }
    let edge = graph.edge(edge_i.unwrap());
    let target = graph.node(edge.target);
    let s = format!(r#"<path d="M {} {} L {} {}"
                    style="fill:none;stroke-width:1;stroke:rgb(0,0,0)" />"#, 
                    source.data.x, source.data.y, 
                    target.data.x, target.data.y);

    edge_to_svg(graph, format!("{}{}", acc, s), target, edge.next, vec!())
}
