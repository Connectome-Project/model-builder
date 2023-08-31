use super::node_type::NodeType;
use crate::pattern::PatternTrait;
use std::fmt::Debug;
use std::fmt::Display;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Eq, PartialOrd, Ord, Default)]
pub struct Node<T>
where
    T: Clone + PartialEq + Ord + PatternTrait + Display + Default + Debug,
{
    pub pattern: T,
    pub node_type: NodeType,
}

impl<T> Node<T>
where
    T: Clone + PartialEq + Ord + PatternTrait + Display + Default + Debug,
{
    pub fn new(pattern: T, node_type: NodeType) -> Self {
        Node { pattern, node_type }
    }
}

impl<T> Display for Node<T>
where
    T: Clone + PartialEq + Ord + PatternTrait + Display + Default + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pattern.clone())
    }
}
