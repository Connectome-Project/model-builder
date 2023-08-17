use super::node::Node;
use crate::pattern::PatternTrait;
use std::fmt::Display;

#[allow(dead_code)]
pub struct ConnectionToNode<'a, T, R>
where
    T: Clone + PartialEq + 'static + Ord + PatternTrait + Display + Default,
{
    pub node: &'a Node<T>,
    pub connection_info: Option<R>,
}
