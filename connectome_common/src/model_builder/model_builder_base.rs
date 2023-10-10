use std::{
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
    sync::mpsc::Sender,
    vec::IntoIter,
};

use petgraph::stable_graph::IndexType;

use crate::{
    arc_model::graph_change_request::GraphChangeRequest,
    pattern::{InnerIterable, PatternTrait},
};

use super::{model_builder_type::ModelBuilderType, TrainingConfig};

pub struct ModelBuilder<Mod, Dat, SomeInnerIterable, E, Pattern, Ix>
where
    E: Error,
    Pattern: Clone + Ord + 'static + Default + PatternTrait + Display + Debug,
    SomeInnerIterable: InnerIterable<Pattern, IntoIter<Pattern>>,
    Dat: Iterator<Item = Result<SomeInnerIterable, E>>,
    Ix: Clone + IndexType,
{
    pub type_of: ModelBuilderType,
    pub config: TrainingConfig,
    pub model: Mod,
    pub data: Dat,
    pub channel: Sender<GraphChangeRequest<Pattern, Ix>>,
    p: PhantomData<Pattern>,
}

impl<Mod, Dat, SomeInnerIterable, E, Pattern, Ix>
    ModelBuilder<Mod, Dat, SomeInnerIterable, E, Pattern, Ix>
where
    E: Error,
    Pattern: Clone + Ord + 'static + Default + PatternTrait + Display + Debug,
    SomeInnerIterable: InnerIterable<Pattern, IntoIter<Pattern>>,
    Dat: Iterator<Item = Result<SomeInnerIterable, E>>,
    Ix: Clone + IndexType,
{
    pub fn new(
        type_of: ModelBuilderType,
        config: TrainingConfig,
        model: Mod,
        data: Dat,
        channel: Sender<GraphChangeRequest<Pattern, Ix>>,
    ) -> Self {
        ModelBuilder {
            type_of,
            config,
            model,
            data,
            channel,
            p: PhantomData,
        }
    }
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
        arc_model::ThreadSafeModel,
        assemble_relative_path,
        model_builder::{model_builder_type::ModelBuilderType, TrainingConfig},
        read_lines, GraphChangeRequest, ModelBuilder, ModelBuilderTrait,
    };

    #[test]
    fn test_model_builder_creation() {
        let (sender, _) = channel::<GraphChangeRequest<String, usize>>();
        let model = ThreadSafeModel::<String, String, usize>::new();

        let combined_path: PathBuf = assemble_relative_path("src/example.txt");
        let lines: Lines<BufReader<File>> = read_lines(combined_path).unwrap();
        let config = TrainingConfig {};
        let model_builder =
            ModelBuilder::new(ModelBuilderType::Builder, config, model, lines, sender);
        assert!(model_builder.is_applicable())
    }
}
