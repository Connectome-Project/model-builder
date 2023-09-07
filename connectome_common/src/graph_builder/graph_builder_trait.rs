use super::GraphBuilder;
use crate::{
    arc_model::{
        graph_change_request::{
            GraphChangeRequest::{AddConnection, AddNode},
            NodeOrIdx,
        },
        model::Model,
        ThreadSafeModel,
    },
    pattern::PatternTrait,
};
use petgraph::stable_graph::{IndexType, NodeIndex};
use std::fmt::{Debug, Display};

pub trait GraphBuilderTrait<PatternContent, Ix>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Ix: Clone + IndexType,
{
    fn build_graph_from_channel(&mut self);
}

impl<PatternContent, Ix> GraphBuilderTrait<PatternContent, Ix>
    for GraphBuilder<PatternContent, Ix, ThreadSafeModel<PatternContent, PatternContent, Ix>>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Ix: Clone + IndexType,
{
    fn build_graph_from_channel(&mut self) {
        if let Some(channel) = &mut self.channel {
            for request in channel.iter() {
                match self.model.data.write() {
                    Ok(mut writer) => match request {
                        AddConnection {
                            from_node,
                            to_node,
                            connection,
                        } => {
                            let from = get_idx_from_node_or_idx(from_node, &writer);
                            let to = get_idx_from_node_or_idx(to_node, &writer);
                            let existing_edge = writer.data.find_edge(from, to);
                            if existing_edge.is_some() {
                                let edge_idx = existing_edge.unwrap();
                                let mut edge_to_modify =
                                    writer.data.edge_weight_mut(edge_idx).unwrap();
                                if edge_to_modify.connection_info.is_some() {
                                    let edge = edge_to_modify.connection_info.as_mut().unwrap();
                                    let content = connection.connection_info.unwrap();
                                    edge.extend(content);
                                }
                            } else {
                                writer.data.add_edge(from, to, connection);
                            }
                        }
                        AddNode(node) => {
                            writer.data.add_node(node);
                        }
                    },
                    Err(e) => {
                        panic!("Could not build graph {}", e);
                    }
                }
            }
        } else {
            panic!("There is no channel awailable, shutting down build process");
        }
    }
}

fn get_idx_from_node_or_idx<PatternContent, Ix>(
    from_node: NodeOrIdx<PatternContent, Ix>,
    writer: &std::sync::RwLockWriteGuard<'_, Model<PatternContent, PatternContent, Ix>>,
) -> NodeIndex<Ix>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Ix: Clone + IndexType,
{
    match from_node {
        NodeOrIdx::Index(idx) => idx,
        NodeOrIdx::Pattern(pattern) => writer
            .data
            .node_indices()
            .find(|idx| writer.data[*idx].pattern == pattern)
            .unwrap(),
    }
}
