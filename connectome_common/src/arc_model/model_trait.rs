use petgraph::stable_graph::{IndexType, NodeIndex};

use crate::{arc_model::node::Node, pattern::PatternTrait};
use std::fmt::{Debug, Display};

use super::{connection::Connection, model::Model};

pub trait ModelTrait<Pattern, NodeDesignator>
where
    Pattern: Clone + PartialEq + 'static + Ord + PatternTrait + Display + Default + Debug,
    NodeDesignator: Debug + Clone,
{
    fn add_vertex(&mut self, node: Node<Pattern>) -> NodeDesignator;
    fn add_connection(
        &mut self,
        content: Connection<Pattern>,
        from: NodeDesignator,
        to: NodeDesignator,
    ) -> ();
}

impl<Pattern, Ix> ModelTrait<Pattern, NodeIndex<Ix>> for Model<Pattern, Pattern, Ix>
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
{
    fn add_vertex(&mut self, node: Node<Pattern>) -> NodeIndex<Ix> {
        self.data.add_node(node)
    }

    fn add_connection(
        &mut self,
        content: Connection<Pattern>,
        from: NodeIndex<Ix>,
        to: NodeIndex<Ix>,
    ) {
        self.data.add_edge(from, to, content);
    }
}
