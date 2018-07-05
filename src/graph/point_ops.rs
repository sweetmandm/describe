use graph::*;
use geometry::*;

impl Graph<Point> {
    pub fn split_edge_mut(&mut self, group_i: GroupIndex, edge_i: EdgeIndex) -> NodeIndex {
        let (a, c) = self.edge(group_i, edge_i);
        let b_point = mid(&self.nodes[a].data, &self.nodes[c].data);
        let b = self.add_node(b_point);
        self.update_edge(group_i, edge_i, (a, b));
        self.add_edge(group_i, (b, c));
        b
    }

    #[allow(dead_code)]
    pub fn split_edge(&mut self, group_i: GroupIndex, edge_i: EdgeIndex) -> (Point, Point, Point) {
        let (a, c) = self.edge(group_i, edge_i);
        let b = mid(&self.nodes[a].data, &self.nodes[c].data);
        ((&self.nodes[a].data).clone(), b, (&self.nodes[c].data).clone())
    }

    pub fn swap_xy(mut self) -> Self {
        for node_i in 0..self.nodes.len() {
            let pos = self.nodes[node_i].data;
            let new_pos = Point::new(pos.y, pos.x, pos.z);
            self.update_node_data(node_i, new_pos);
        }
        self
    }

    pub fn shift(mut self, amount: Vector) -> Self {
        for node_i in 0..self.nodes.len() {
            let pos = self.nodes[node_i].data;
            self.update_node_data(node_i, pos + amount);
        }
        self
    }
}

fn mid(a: &Point, b: &Point) -> Point {
    Point::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0, (a.z + b.z) / 2.0)
}
