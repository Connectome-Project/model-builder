use std::cmp::Ordering;
use std::error::Error;
use std::marker::PhantomData;
use std::str::Chars;

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

trait InnerIterable<Item: Clone + Ord + 'static> {
    type Iterable<'a>: Iterator<Item = Item>
    where
        Self: 'a;

    fn get_inner_iterable(&self) -> Self::Iterable<'_>;
}

impl InnerIterable<char> for String {
    type Iterable<'a> = Chars<'a> where Self: 'a;

    fn get_inner_iterable(&self) -> Self::Iterable<'_> {
        self.chars()
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

trait ModelBuilderTrait<D> {
    fn performAction(&mut self);
    fn isApplicable(&self) -> bool;
}

impl<'a, 'b, SomeInnerIterable, Additional, Dat, E, DataPoint> ModelBuilderTrait<Dat>
    for ModelBuilder<
        ThreadSafeModel<'a, 'b, DataPoint, Additional>,
        Dat,
        SomeInnerIterable,
        E,
        DataPoint,
    >
where
    E: Error,
    DataPoint: Clone + Ord + 'static,
    Additional: 'a + 'static,
    SomeInnerIterable: InnerIterable<DataPoint>,
    Dat: Iterator<Item = Result<SomeInnerIterable, E>>,
{
    fn isApplicable(&self) -> bool {
        true
    }
    fn performAction(&mut self) {
        for chunk in &mut self.data {
            let read = self.model.model.read().unwrap();
            let nodes = read.get_nodes();
            let start_nodes = nodes.iter().find(|y| y.node_type == NodeType::Start);

            let chunk_res: SomeInnerIterable = chunk.unwrap();
            let chunk_internal_iterator = chunk_res.get_inner_iterable();

            for elem in chunk_internal_iterator {
                let found_node: Option<&crate::arc_model::Node<DataPoint>> =
                    nodes
                        .iter()
                        .find(|node: &&crate::arc_model::Node<DataPoint>| {
                            node.pattern.cmp(&elem) == Ordering::Equal
                        });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ModelBuilder, ModelBuilderTrait, ModelBuilderType, TrainingConfig};
    use crate::{
        arc_model::ThreadSafeModel,
        input::{DataContainer, InputTwoWayIterable},
        read_file::{assemble_relative_path, read_lines},
    };

    #[test]
    fn test_model_builder_creation() {
        let input = vec!["This ist the first line.", "This is the second line."];
        let type_of = ModelBuilderType::Builder;
        let config = TrainingConfig {};
        let model = ThreadSafeModel::<&str, ()>::new();
        let combined_path = assemble_relative_path("src/example.txt");
        let mut lines: std::io::Lines<std::io::BufReader<std::fs::File>> =
            read_lines(combined_path).unwrap();
        // lines.next().builder.performAction();

        let builder = ModelBuilder::new(type_of, config, model, lines);
    }
}
