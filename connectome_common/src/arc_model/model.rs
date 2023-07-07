use std::collections::BTreeMap;

use super::node_type::NodeType;

pub struct Node<T>
where
    T: Clone + 'static,
{
    pattern: T,
    node_type: NodeType,
}

struct ConnectionToNode<T, R>
where
    T: Clone + 'static,
{
    node: Vec<Node<T>>,
    connection_info: Option<R>,
}

pub struct Model<T, R>
where
    T: Clone + 'static,
    R: 'static,
{
    nodes: Vec<Node<T>>,
    connections: BTreeMap<Node<T>, ConnectionToNode<T, R>>,
}

impl<T, R> Model<T, R>
where
    T: Clone + 'static,
    R: 'static,
{
    pub fn new() -> Self {
        Model {
            nodes: Vec::new(),
            connections: BTreeMap::new(),
        }
    }
}

pub trait ModelTrait<T, R>
where
    T: Clone + 'static,
    R: 'static,
{
    fn get_nodes(&self) -> &Vec<Node<T>>;
}

impl<T: Clone, R> ModelTrait<T, R> for Model<T, R> {
    fn get_nodes(&self) -> &Vec<Node<T>> {
        &self.nodes
    }
}
