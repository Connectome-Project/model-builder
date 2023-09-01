use super::connection::Connection;
use super::node::Node;
use crate::pattern::PatternTrait;
use petgraph::stable_graph::{IndexType, StableGraph};
use petgraph::Directed;
use std::fmt::{Debug, Display};

pub struct Model<T, R, Ix>
where
    T: Clone + PartialEq + 'static + Ord + PatternTrait + Display + Default + Debug,
    R: Clone
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Display
        + Debug
        + Default
        + PatternTrait
        + 'static,
    Ix: Clone + Debug,
{
    pub data: StableGraph<Node<T>, Connection<R>, Directed, Ix>,
}

impl<T, R, Ix> Model<T, R, Ix>
where
    T: Clone + PartialEq + 'static + Ord + PatternTrait + Display + Default + Debug,
    R: Clone
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Display
        + Debug
        + Default
        + PatternTrait
        + 'static,
    Ix: Clone + Debug + PartialOrd + Eq + IndexType + Ord + Default,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Model {
            data: StableGraph::<Node<T>, Connection<R>, Directed, Ix>::default(),
        }
    }
    pub fn get_data(&self) -> &StableGraph<Node<T>, Connection<R>, Directed, Ix> {
        &self.data
    }
    pub fn get_data_mut(&mut self) -> &mut StableGraph<Node<T>, Connection<R>, Directed, Ix> {
        &mut self.data
    }
}
