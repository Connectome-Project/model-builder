use crate::arc_model::connection::ConnectionToNode;
use crate::arc_model::Node;
use crate::pattern::PatternTrait;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Display;
use std::iter::Peekable;
use std::marker::PhantomData;
use std::vec::IntoIter;

use super::arc_model::ThreadSafeModel;

struct ModelBuilder<Mod, Dat, SomeInnerIterable, E, Pattern>
where
    E: Error,
    Pattern: Clone + Ord + 'static + Default,
    SomeInnerIterable: InnerIterable<Pattern, IntoIter<Pattern>>,
    Dat: Iterator<Item = Result<SomeInnerIterable, E>>,
{
    type_of: ModelBuilderType,
    config: TrainingConfig,
    model: Mod,
    data: Dat,
    p: PhantomData<Pattern>,
}

impl<Mod, Dat, SomeInnerIterable, E, Pattern> ModelBuilder<Mod, Dat, SomeInnerIterable, E, Pattern>
where
    E: Error,
    Pattern: Clone + Ord + 'static + Default,
    SomeInnerIterable: InnerIterable<Pattern, IntoIter<Pattern>>,
    Dat: Iterator<Item = Result<SomeInnerIterable, E>>,
{
    fn new(type_of: ModelBuilderType, config: TrainingConfig, model: Mod, data: Dat) -> Self {
        ModelBuilder {
            type_of,
            config,
            model,
            data,
            p: PhantomData,
        }
    }
}

trait InnerIterable<It: Clone + Ord + 'static, Iter: Iterator<Item = It>> {
    fn get_inner_iterable(&self) -> Iter;
}

impl InnerIterable<String, IntoIter<String>> for String {
    fn get_inner_iterable(&self) -> IntoIter<String> {
        let df = self.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        let iter: std::vec::IntoIter<String> = df.into_iter();
        return iter;
    }
}

pub enum ModelBuilderType {
    Builder,
    Stereotype,
    StereotypeEvolution,
    Adoption,
    GarbageCollector,
}

pub struct TrainingConfig {}

trait ModelBuilderTrait {
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

                let mut previous_node: Option<Node<PatternContent>> = None;
                find_longest_pattern(
                    combined_nodes,
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

struct LongestPattern<'a, Pattern>
where
    Pattern: Clone + Ord + 'static + PatternTrait + Display + Default,
{
    matching_node: &'a Node<Pattern>,
    pattern_so_far: Pattern,
}

fn find_longest_pattern<'a, PatternContent>(
    nodes: Vec<&'a Node<PatternContent>>,
    mut data_iterator: Peekable<IntoIter<PatternContent>>,
    mut pattern_so_far: PatternContent,
) -> Option<LongestPattern<'a, PatternContent>>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default,
{
    if let Some(d) = data_iterator.peek() {
        let extended_pattern_so_far = pattern_so_far.concat(d);
        let mut remaining: Vec<&Node<PatternContent>> = nodes
            .iter()
            .map(|f| *f)
            .filter(|node| {
                let reg = regex::Regex::new(&format!("r{}", extended_pattern_so_far)).unwrap();
                return node.pattern.match_against(reg);
            })
            .collect::<Vec<&Node<PatternContent>>>();

        remaining.sort_by(|a, b| b.cmp(a));

        if let Some(first_node) = remaining.get(0) {
            if first_node.pattern.len() > extended_pattern_so_far.len() && !nodes.is_empty() {
                return find_longest_pattern(remaining, data_iterator, pattern_so_far);
            }
        }
        // return Some(LongestPattern {
        //     matching_node: nodes.get(0),
        //     pattern_so_far: pattern_so_far.clone(),
        // });
    } else {
        if pattern_so_far == PatternContent::default() || nodes.is_empty() {
            return None;
        }
        return Some(LongestPattern {
            matching_node: nodes.get(0).unwrap(),
            pattern_so_far: pattern_so_far.clone(),
        });
    }
    return Some(LongestPattern {
        matching_node: nodes.get(0).unwrap(),
        pattern_so_far: pattern_so_far.clone(),
    });
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
    use std::{
        fs::File,
        io::{BufReader, Lines},
        path::PathBuf,
    };

    use super::{
        find_longest_pattern, ModelBuilder, ModelBuilderTrait, ModelBuilderType, TrainingConfig,
    };
    use crate::{
        arc_model::NodeType,
        arc_model::{Node, ThreadSafeModel},
        model_builder::{InnerIterable, LongestPattern},
        read_file::{assemble_relative_path, read_lines},
    };

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

    #[test]
    fn test_find_longest_pattern() {
        let node1 = Node::new("h".to_string(), NodeType::Start);
        let node2 = Node::new("h*".to_string(), NodeType::Start);
        let node3 = Node::new("l".to_string(), NodeType::Start);

        let nodes = vec![&node1, &node2, &node3];
        let nodes_cloned = nodes.clone();
        let data = "hello"
            .to_string()
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .into_iter();

        let res: LongestPattern<'_, String> =
            find_longest_pattern(nodes, data.peekable(), "".to_string()).unwrap();
        assert_eq!(nodes_cloned.get(2).unwrap(), &res.matching_node);
        assert_eq!("hel", res.pattern_so_far);
    }
}
