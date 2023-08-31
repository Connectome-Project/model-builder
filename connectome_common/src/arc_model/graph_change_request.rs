use super::{connection::Connection, Node};
use crate::pattern::PatternTrait;
use petgraph::stable_graph::NodeIndex;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum NodeOrIndex<PatternContent, Ix = u32>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
{
    Index(NodeIndex<Ix>),
    Pattern(PatternContent),
}

#[derive(Debug, Clone)]
pub enum GraphChangeRequest<PatternContent, Ix = u32>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
{
    AddNode(Node<PatternContent>),
    AddConnection {
        from_node: NodeOrIndex<PatternContent, Ix>,
        to_node: NodeOrIndex<PatternContent, Ix>,
        connection: Connection,
    },
}
