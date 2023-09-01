use std::{
    error::Error,
    fmt::{Debug, Display},
    hash::Hash,
    vec::IntoIter,
};

use petgraph::stable_graph::IndexType;

use crate::{
    arc_model::Node,
    arc_model::{
        connection::{Connection, ConnectionInfo},
        graph_change_request::{GraphChangeRequest, NodeOrIdx},
        ThreadSafeModel,
    },
    pattern::{
        find_longest_pattern, CloneableOption, InnerIterable, LongestPatternResult,
        NodeWithOptionalIdx, PatternTrait,
    },
};

use super::model_builder_base::ModelBuilder;

pub trait ModelBuilderTrait {
    fn perform_action(&mut self);
    fn is_applicable(&self) -> bool;
}

impl<'a, 'b, SomeInnerIterable, Dat, E, PatternContent, Ix> ModelBuilderTrait
    for ModelBuilder<
        ThreadSafeModel<PatternContent, PatternContent, Ix>,
        Dat,
        SomeInnerIterable,
        E,
        PatternContent,
        Ix,
    >
where
    E: Error,
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    SomeInnerIterable: InnerIterable<PatternContent, IntoIter<PatternContent>>,
    Dat: Iterator<Item = Result<SomeInnerIterable, E>>,
    Ix: Clone + Debug + PartialOrd + Eq + Hash + IndexType + Ord + Default,
{
    fn is_applicable(&self) -> bool {
        true
    }
    fn perform_action(&mut self) {
        let mut nodes_to_add: Vec<Node<PatternContent>> = Vec::new();
        let Ok(read) = self.model.model.read() else{
            panic!("Could not read model");
        };
        let data = read.get_data();

        for chunk_res in &mut self.data {
            //merge to be added and existing nodes
            let nodes_to_add_clone = nodes_to_add.clone();
            let nodes_to_add_ref = nodes_to_add_clone
                .iter()
                .map(|e| NodeWithOptionalIdx {
                    node: &e,
                    index: CloneableOption::new_none(),
                })
                .collect::<Vec<NodeWithOptionalIdx<'_, PatternContent, Ix>>>();

            let mut node_weights = data
                .node_indices()
                .into_iter()
                .map(|e| NodeWithOptionalIdx {
                    node: data.node_weight(e).unwrap(),
                    index: CloneableOption::new_some(e),
                })
                .chain(nodes_to_add_ref)
                .collect::<Vec<NodeWithOptionalIdx<'_, PatternContent, Ix>>>();

            if let Ok(chunk) = chunk_res {
                let chunk_internal_iterator = chunk.get_inner_iterable();
                let mut iter_to_go_through = chunk_internal_iterator.peekable();

                let mut last_node_found: Option<NodeWithOptionalIdx<PatternContent, Ix>> = None;
                let mut last_node_content: Node<PatternContent>;
                loop {
                    let res = find_longest_pattern(
                        node_weights.clone(),
                        None,
                        iter_to_go_through,
                        PatternContent::default(),
                    );

                    match res {
                        LongestPatternResult::ResultWithIter(result) => {
                            let matched_node: NodeWithOptionalIdx<'_, PatternContent, Ix> =
                                result.matching_node;
                            let pattern_to_connection = result.pattern_so_far;

                            if let Some(last_found_node_or_idx) = last_node_found.clone() {
                                let added_connection = GraphChangeRequest::AddConnection {
                                    from_node: NodeOrIdx::from(last_found_node_or_idx),
                                    to_node: NodeOrIdx::from(matched_node.clone()),
                                    connection: Connection::build_from_content(ConnectionInfo {
                                        label: pattern_to_connection.clone(),
                                    }),
                                };
                                let _ = self.channel.send(added_connection);
                            }

                            //set values for continuation
                            iter_to_go_through = result.remaining_iter;
                            last_node_found = Some(matched_node);
                        }
                        LongestPatternResult::Iter(mut it) => {
                            if it.len() > 0 {
                                let pattern = it.next().unwrap();
                                let node_added = Node {
                                    pattern,
                                    node_type: crate::arc_model::NodeType::Generated,
                                };
                                let node_added_cloned = node_added.clone();

                                let added_graph = GraphChangeRequest::AddNode(node_added);
                                let _ = self.channel.send(added_graph); //No error handling

                                if let Some(node_with_opt_idx) = last_node_found {
                                    let node_or_idx = NodeOrIdx::from(node_with_opt_idx);
                                    let added_connection = GraphChangeRequest::AddConnection {
                                        from_node: node_or_idx,
                                        to_node: NodeOrIdx::Pattern(
                                            node_added_cloned.pattern.clone(),
                                        ),
                                        connection: Connection::build_from_content(
                                            ConnectionInfo {
                                                label: node_added_cloned.pattern.clone(),
                                            },
                                        ),
                                    };
                                    let _ = self.channel.send(added_connection);
                                }

                                //set values for continuation
                                iter_to_go_through = it;
                                last_node_content = node_added_cloned;
                                last_node_found = Some(NodeWithOptionalIdx {
                                    index: CloneableOption::new_none(),
                                    node: &last_node_content,
                                });
                            } else {
                                break;
                            }
                        }
                    };
                }
            }
        }
    }
}

fn combine_nodes<'a, PatternContent>(
    nodes: &'a Vec<Node<PatternContent>>,
    nodes_to_add: &'a Vec<Node<PatternContent>>,
) -> Vec<&'a Node<PatternContent>>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
{
    nodes
        .iter()
        .chain(nodes_to_add.iter())
        .collect::<Vec<&Node<PatternContent>>>()
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, Lines},
        path::PathBuf,
        sync::mpsc::channel,
    };

    use crate::{
        arc_model::{graph_change_request::GraphChangeRequest, Node, ThreadSafeModel},
        model_builder::{
            model_builder_base::ModelBuilder, model_builder_type::ModelBuilderType, TrainingConfig,
        },
        read_file::{assemble_relative_path, read_lines},
    };

    use super::ModelBuilderTrait;

    #[test]
    fn test_model_builder_creation() {
        let input = vec!["This ist the first line.", "This is the second line."];
        let type_of: ModelBuilderType = ModelBuilderType::Builder;
        let config: TrainingConfig = TrainingConfig {};
        let model = ThreadSafeModel::<String, String, usize>::new();

        {
            let mut writeable_model = model.model.write().unwrap();
            writeable_model.data.add_node(Node {
                pattern: "".to_string(),
                node_type: crate::arc_model::NodeType::Start,
            });
        }

        let (tx, rx) = channel::<GraphChangeRequest<String, usize>>();

        let combined_path: PathBuf = assemble_relative_path("src/example.txt");
        let mut lines: Lines<BufReader<File>> = read_lines(combined_path).unwrap();

        let mut builder = ModelBuilder::new(type_of, config, model, lines, tx);
        if builder.is_applicable() {
            builder.perform_action();
        }
    }
}
