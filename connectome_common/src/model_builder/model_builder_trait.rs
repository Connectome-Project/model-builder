use std::{
    error::Error,
    fmt::{Debug, Display},
    vec::IntoIter,
};

use crate::{
    arc_model::Node,
    arc_model::{graph_change_request::GraphChangeRequest, ThreadSafeModel},
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
    for ModelBuilder<ThreadSafeModel<PatternContent>, Dat, SomeInnerIterable, E, PatternContent, Ix>
where
    E: Error,
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    SomeInnerIterable: InnerIterable<PatternContent, IntoIter<PatternContent>>,
    Dat: Iterator<Item = Result<SomeInnerIterable, E>>,
    Ix: Clone,
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

        // let first_index = data.node_indices().next().unwrap();
        // let start_node = data
        //     .node_weight(first_index)
        //     .iter()
        //     .filter(|w| w.node_type == NodeType::Start)
        //     .next()
        //     .unwrap();

        for chunk_res in &mut self.data {
            //merge to be added and existing nodes
            let nodes_to_add_clone = nodes_to_add.clone();
            let nodes_to_add_ref = nodes_to_add_clone
                .iter()
                .map(|e| NodeWithOptionalIdx {
                    node: &e,
                    index: CloneableOption::new_none(),
                })
                .collect::<Vec<NodeWithOptionalIdx<'_, PatternContent, usize>>>();
            let mut node_weights = data
                .node_indices()
                .into_iter()
                .map(|e| NodeWithOptionalIdx {
                    node: data.node_weight(e).unwrap(),
                    index: CloneableOption::new_some(e),
                })
                .chain(nodes_to_add_ref)
                .collect::<Vec<NodeWithOptionalIdx<'_, PatternContent, usize>>>();

            if let Ok(chunk) = chunk_res {
                let chunk_internal_iterator = chunk.get_inner_iterable();

                // let mut previous_node: Option<Node<PatternContent>> = None;
                // let last_node_found: Option<&Node<PatternContent>> = None;
                let mut iter_to_go_through = chunk_internal_iterator.peekable();

                loop {
                    let res = find_longest_pattern(
                        node_weights.clone(),
                        None,
                        iter_to_go_through,
                        PatternContent::default(),
                    );

                    match res {
                        LongestPatternResult::ResultWithIter(result) => {
                            let node = result.matching_node;
                            iter_to_go_through = result.remaining_iter;
                        }
                        LongestPatternResult::Iter(mut it) => {
                            if it.len() > 0 {
                                let pattern = it.next().unwrap();
                                let node_added = Node {
                                    pattern,
                                    node_type: crate::arc_model::NodeType::Generated,
                                };
                                let added_graph = GraphChangeRequest::AddNode(node_added);
                                self.channel.send(added_graph);
                                // let connection_added = Connection {
                                //     connection_info: Some(vec![ConnectionInfo { label: chunk }]),
                                // };
                                // nodes_to_add.push(Chan {
                                //     new_node: node_added,
                                //     connection_from_node_index: ,
                                //     connection: connection_added,
                                // });

                                // match last_node_found {
                                //     Some(last_node) => {
                                //         let connections: Vec<
                                //             ConnectionToNode<'_, PatternContent, ()>,
                                //         > = vec![ConnectionToNode {
                                //             node: &node_added2,
                                //             connection_info: Some(()),
                                //         }];
                                //         connections_to_add.insert(&last_node, connections);
                                //     }
                                //     None => {}
                                // };

                                // last_node_found = Some(node_added);
                            }
                            break;
                        } // LongestPatternResult::None => {}
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

    use petgraph::stable_graph::NodeIndex;

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
        let model = ThreadSafeModel::<String>::new();

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
