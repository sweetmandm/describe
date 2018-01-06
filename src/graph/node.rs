pub type NodeIndex = usize;

pub struct Node<T> {
    pub data: T,
    pub dead: bool
}

impl<T: PartialEq> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node { data, dead: false }
    }
}

