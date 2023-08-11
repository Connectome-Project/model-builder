use std::error::Error;
use std::str::Chars;

use super::arc_model::NodeType;
use super::arc_model::ThreadSafeModel;

struct ModelBuilder<Mod, Dat, It, P>
where
    P: Error,
    It: Clone + Ord + 'static,
    Dat: Iterator<Item = Result<It, P>>,
{
    type_of: ModelBuilderType,
    config: TrainingConfig,
    model: Mod,
    data: Dat,
}

impl<Mod, Dat, It, P> ModelBuilder<Mod, Dat, It, P>
where
    P: Error,
    It: Clone + Ord + 'static,
    Dat: Iterator<Item = Result<It, P>>,
{
    fn new(type_of: ModelBuilderType, config: TrainingConfig, model: Mod, data: Dat) -> Self {
        ModelBuilder {
            type_of,
            config,
            model,
            data,
        }
    }
}

trait InnerIterable {
    type Iterable<'a>
    where
        Self: 'a;

    fn get_inner_iterable(&self) -> Self::Iterable<'_>;
}

impl InnerIterable for String {
    type Iterable<'a> = Chars<'a>;

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
    fn performAction(&self);
    fn isApplicable(&self) -> bool;
}

impl<'a, 'b, It, Additional, Dat, P> ModelBuilderTrait<Dat>
    for ModelBuilder<ThreadSafeModel<'a, 'b, It, Additional>, Dat, It, P>
where
    P: Error,
    It: Clone + Ord + 'static,
    Additional: 'a + 'static,
    Dat: Iterator<Item = Result<It, P>>,
{
    fn isApplicable(&self) -> bool {
        true
    }
    fn performAction(&self) {
        self.data.for_each(|x| {
            let d = x.unwrap();
            let read = self.model.model.read().unwrap();
            let nodes = read.get_nodes();
            let start_nodes = nodes.iter().find(|y| y.node_type == NodeType::Start);
        })
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
