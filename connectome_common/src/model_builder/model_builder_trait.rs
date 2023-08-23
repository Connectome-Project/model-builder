use std::{collections::BTreeMap, error::Error, fmt::Display, vec::IntoIter};

use crate::{
    arc_model::{connection::ConnectionToNode, Node, ThreadSafeModel},
    pattern::{find_longest_pattern, InnerIterable, PatternTrait},
};

use super::model_builder_base::ModelBuilder;

pub trait ModelBuilderTrait {
    fn perform_action(&mut self);
    fn is_applicable(&self) -> bool;
}

impl<'a, 'b, SomeInnerIterable, Additional, Dat, E, PatternContent> ModelBuilderTrait
    for ModelBuilder<
        ThreadSafeModel<'a, 'b, PatternContent, Additional>,
        Dat,
        SomeInnerIterable,
        E,
        PatternContent,
    >
where
    E: Error,
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default,
    Additional: 'a + 'static,
    SomeInnerIterable: InnerIterable<PatternContent, IntoIter<PatternContent>>,
    Dat: Iterator<Item = Result<SomeInnerIterable, E>>,
{
    fn is_applicable(&self) -> bool {
        true
    }
    fn perform_action(&mut self) {
        let mut nodes_to_add: Vec<Node<PatternContent>> = Vec::new();
        let mut connections_to_add: BTreeMap<
            &'b Node<PatternContent>,
            Vec<ConnectionToNode<'a, PatternContent, ()>>,
        >;
        let read = self.model.model.read().unwrap();
        let nodes = read.get_nodes();

        for chunk_res in &mut self.data {
            if let Ok(chunk) = chunk_res {
                let combined_nodes = combine_nodes(nodes, &nodes_to_add);
                let chunk_internal_iterator = chunk.get_inner_iterable();
                // let total_chunk = chunk.get_inner_iterable().reduce(|acc, e| acc.concat(&e));
                // let chunk.

                let mut previous_node: Option<Node<PatternContent>> = None;
                find_longest_pattern(
                    combined_nodes,
                    None,
                    chunk_internal_iterator.peekable(),
                    PatternContent::default(),
                );

                // pattern_related_nodes
                // while chunk_internal_iterator.next()
                // for elem in chunk_internal_iterator {
                //     if previous_node == None {
                //         //find in model
                //         let mut found_nodes = (collect)
                //             .iter()
                //             .filter(|node: &&arc_model::Node<PatternContent>| {
                //                 let reg = regex::Regex::new(&format!("r{}", elem)).unwrap();
                //                 return node.pattern.match_against(reg);
                //             })
                //             .collect::<Vec<&Node<PatternContent>>>()
                //             .sort_by(|a, b| b.cmp(a));

                //         //find it in new additions
                //         if let None = found_node {
                //             let possible_node = nodes_to_add
                //                 .iter_mut()
                //                 .find(|node: &&mut arc_model::Node<PatternContent>| {
                //                     node.pattern.cmp(&elem) == Ordering::Equal
                //                 })
                //                 .cloned();
                //             found_node = possible_node;
                //         }

                //         if let None = found_node {
                //             nodes_to_add.push(Node {
                //                 pattern: elem.clone(),
                //                 node_type: NodeType::Generated,
                //             })
                //         }

                //         if let Some(n) = found_node {
                //             // if !read.connections_from.contains_key(n) {}
                //         }
                //     }
                // }
            }
        }
    }
}

fn combine_nodes<'a, PatternContent>(
    nodes: &'a Vec<Node<PatternContent>>,
    nodes_to_add: &'a Vec<Node<PatternContent>>,
) -> Vec<&'a Node<PatternContent>>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default,
{
    nodes
        .iter()
        .chain(nodes_to_add.iter())
        .collect::<Vec<&Node<PatternContent>>>()
}

#[cfg(test)]
mod tests {

    // use super::find_longest_pattern;
    // use crate::{arc_model::Node, arc_model::NodeType, pattern::LongestPattern};

    // #[test]
    // fn test_model_builder_creation() {
    //     let input = vec!["This ist the first line.", "This is the second line."];
    //     let type_of: ModelBuilderType = ModelBuilderType::Builder;
    //     let config: TrainingConfig = TrainingConfig {};
    //     let model = ThreadSafeModel::<'_, '_, String, ()>::new();
    //     let combined_path: PathBuf = assemble_relative_path("src/example.txt");
    //     let mut lines: Lines<BufReader<File>> = read_lines(combined_path).unwrap();

    //     let mut builder = ModelBuilder::new(type_of, config, model, lines);
    //     if (builder.is_applicable()) {
    //         builder.perform_action();
    //     }
    // }
}
