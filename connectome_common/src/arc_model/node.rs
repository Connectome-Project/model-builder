use super::node_type::NodeType;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Eq, PartialOrd, Ord)]
pub struct Node<T>
where
    T: Clone + PartialEq + Ord,
{
    pub pattern: T,
    pub node_type: NodeType,
}

impl<T> Node<T>
where
    T: Clone + PartialEq + Ord,
{
    pub fn new(pattern: T, node_type: NodeType) -> Self {
        Node { pattern, node_type }
    }
}
