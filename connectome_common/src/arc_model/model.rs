use super::connection::Connection;
use super::node::Node;
use crate::pattern::PatternTrait;
use petgraph::stable_graph::StableGraph;
use petgraph::Directed;
use std::fmt::{Debug, Display};

pub struct Model<T, R>
where
    T: Clone + PartialEq + 'static + Ord + PatternTrait + Display + Default + Debug,
    R: Clone + PartialEq + Eq + Ord + PartialOrd + Display + Debug + Default,
{
    pub data: StableGraph<Node<T>, Connection<R>, Directed, usize>,
}

impl<T, R> Model<T, R>
where
    T: Clone + PartialEq + 'static + Ord + PatternTrait + Display + Default + Debug,
    R: Clone + PartialEq + Eq + Ord + PartialOrd + Display + Debug + Default,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Model {
            data: StableGraph::<Node<T>, Connection<R>, Directed, usize>::default(),
        }
    }
    pub fn get_data(&self) -> &StableGraph<Node<T>, Connection<R>, Directed, usize> {
        &self.data
    }
    pub fn get_data_mut(&mut self) -> &mut StableGraph<Node<T>, Connection<R>, Directed, usize> {
        &mut self.data
    }
}
