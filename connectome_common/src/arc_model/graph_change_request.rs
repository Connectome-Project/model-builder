use super::{connection::Connection, Node};
use crate::pattern::{CloneableOption, NodeWithOptionalIdx, PatternTrait};
use petgraph::stable_graph::{IndexType, NodeIndex};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum NodeOrIdx<PatternContent, Ix>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Ix: IndexType + Clone,
{
    Index(NodeIndex<Ix>),
    Pattern(PatternContent),
}

impl<'a, Pattern, Ix> From<NodeWithOptionalIdx<'a, Pattern, Ix>> for NodeOrIdx<Pattern, Ix>
where
    Pattern: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Ix: Clone + IndexType,
{
    fn from(opt: NodeWithOptionalIdx<'a, Pattern, Ix>) -> Self {
        match opt.index {
            CloneableOption(Some(idx)) => NodeOrIdx::Index(idx),
            CloneableOption(None) => NodeOrIdx::Pattern(opt.node.pattern.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GraphChangeRequest<PatternContent, Ix>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Ix: IndexType + Clone,
{
    AddNode(Node<PatternContent>),
    AddConnection {
        from_node: NodeOrIdx<PatternContent, Ix>,
        to_node: NodeOrIdx<PatternContent, Ix>,
        connection: Connection<PatternContent>,
    },
}
