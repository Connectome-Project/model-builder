use std::{
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
    sync::mpsc::Sender,
    vec::IntoIter,
};

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
    Ix: Clone,
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
    Ix: Clone,
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
    use crate::{
        arc_model::ThreadSafeModel,
        model_builder::{model_builder_type::ModelBuilderType, TrainingConfig},
    };

    #[test]
    fn test_model_builder_creation() {
        let input = vec!["This ist the first line.", "This is the second line."];
        let type_of: ModelBuilderType = ModelBuilderType::Builder;
        let config: TrainingConfig = TrainingConfig {};
        let model = ThreadSafeModel::<String, String>::new();

        assert!(!model.model.is_poisoned())
    }
}
