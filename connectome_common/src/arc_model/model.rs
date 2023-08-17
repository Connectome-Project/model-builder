use super::connection::ConnectionToNode;
use super::node::Node;
use crate::pattern::PatternTrait;
use std::collections::BTreeMap;
use std::fmt::Display;

pub struct Model<'a, 'b, T, R>
where
    T: Clone + PartialEq + 'static + Ord + PatternTrait + Display + Default,
    R: 'static,
{
    pub nodes: Vec<Node<T>>,
    #[allow(dead_code)]
    pub connections_from: BTreeMap<&'b Node<T>, Vec<ConnectionToNode<'a, T, R>>>,
}

impl<'a, 'b, T, R> Model<'a, 'b, T, R>
where
    T: Clone + PartialEq + 'static + Ord + PatternTrait + Display + Default,
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
