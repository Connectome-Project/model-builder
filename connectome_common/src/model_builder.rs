use std::cmp::Ordering;
use std::error::Error;
use std::marker::PhantomData;
use std::str::Chars;
use std::vec::IntoIter;

use super::arc_model::NodeType;
use super::arc_model::ThreadSafeModel;

struct ModelBuilder<Mod, Dat, SomeInnerIterable, E, Pattern>
where
    E: Error,
    Pattern: Clone + Ord + 'static,
    SomeInnerIterable: InnerIterable<Pattern>,
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
    Pattern: Clone + Ord + 'static,
    SomeInnerIterable: InnerIterable<Pattern>,
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

trait InnerIterable<It: Clone + Ord + 'static> {
    type Iterable: Iterator<Item = It>;

    fn get_inner_iterable(&self) -> Self::Iterable;
}

impl InnerIterable<String> for String {
    type Iterable = IntoIter<String>;

    fn get_inner_iterable(&self) -> Self::Iterable {
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

impl<'a, 'b, SomeInnerIterable, Additional, Dat, E, Pattern> ModelBuilderTrait
    for ModelBuilder<
        ThreadSafeModel<'a, 'b, Pattern, Additional>,
        Dat,
        SomeInnerIterable,
        E,
        Pattern,
    >
where
    E: Error,
    Pattern: Clone + Ord + 'static,
    Additional: 'a + 'static,
    SomeInnerIterable: InnerIterable<Pattern>,
    Dat: Iterator<Item = Result<SomeInnerIterable, E>>,
{
    fn is_applicable(&self) -> bool {
        true
    }
    fn perform_action(&mut self) {
        for chunk in &mut self.data {
            let read = self.model.model.read().unwrap();
            let nodes = read.get_nodes();
            let start_nodes = nodes.iter().find(|y| y.node_type == NodeType::Start);

            let chunk_res: SomeInnerIterable = chunk.unwrap();
            let chunk_internal_iterator = chunk_res.get_inner_iterable();

            for elem in chunk_internal_iterator {
                let found_node: Option<&crate::arc_model::Node<Pattern>> =
                    nodes
                        .iter()
                        .find(|node: &&crate::arc_model::Node<Pattern>| {
                            node.pattern.cmp(&elem) == Ordering::Equal
                        });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, Error, Lines},
        path::PathBuf,
    };

    use super::{ModelBuilder, ModelBuilderTrait, ModelBuilderType, TrainingConfig};
    use crate::{
        arc_model::ThreadSafeModel,
        read_file::{assemble_relative_path, read_lines},
    };

    #[test]
    fn test_model_builder_creation() {
        let input = vec!["This ist the first line.", "This is the second line."];
        let type_of: ModelBuilderType = ModelBuilderType::Builder;
        let config: TrainingConfig = TrainingConfig {};
        let model = ThreadSafeModel::<'_, '_, String, ()>::new();
        let combined_path: PathBuf = assemble_relative_path("src/example.txt");
        let mut lines: Lines<BufReader<File>> = read_lines(combined_path).unwrap();

        let mut builder = ModelBuilder::new(type_of, config, model, lines);
        if (builder.is_applicable()) {
            builder.perform_action();
        }
    }
}
