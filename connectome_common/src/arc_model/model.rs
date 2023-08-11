use std::collections::BTreeMap;

use super::connection::ConnectionToNode;
use super::node::Node;

pub struct Model<'a, 'b, T, R>
where
    T: Clone + PartialEq + 'static + Ord,
    R: 'static,
{
    nodes: Vec<Node<T>>,
    #[allow(dead_code)]
    connections_from: BTreeMap<&'b Node<T>, Vec<ConnectionToNode<'a, T, R>>>,
}

impl<'a, 'b, T, R> Model<'a, 'b, T, R>
where
    T: Clone + PartialEq + 'static + Ord,
    R: 'static,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Model {
            nodes: Vec::new(),
            connections_from: BTreeMap::new(),
        }
    }

    pub fn get_nodes(&self) -> &Vec<Node<T>> {
        &self.nodes
    }
}
