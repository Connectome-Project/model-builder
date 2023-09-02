use petgraph::stable_graph::IndexType;

use crate::{arc_model::node::Node, pattern::PatternTrait};
use std::fmt::{Debug, Display};

use super::{connection::Connection, model::Model};

pub trait ModelTrait<Pattern, NodeDesignator>
where
    Pattern: Clone + PartialEq + 'static + Ord + PatternTrait + Display + Default + Debug,
    NodeDesignator: Debug + Clone,
{
    fn add_node(&mut self, node: Node<Pattern>) -> ();
    fn add_connection(
        &mut self,
        node: Connection<Pattern>,
        from: NodeDesignator,
        to: NodeDesignator,
    ) -> ();
}

impl<Pattern, Ix, T> ModelTrait<Pattern, T> for Model<Pattern, Pattern, Ix>
where
    Pattern: Clone
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Display
        + Debug
        + Default
        + PatternTrait
        + 'static,
    Ix: IndexType + Clone,
    T: Clone + Debug,
{
    fn add_node(&mut self, node: Node<Pattern>) {
        let dfs: &mut petgraph::stable_graph::StableGraph<
            Node<Pattern>,
            Connection<Pattern>,
            petgraph::Directed,
            Ix,
        > = &mut self.data;
        let adf = self.data.node_weights();
    }

    fn add_connection(&mut self, node: Connection<Pattern>, from: T, to: T) -> () {}
}
