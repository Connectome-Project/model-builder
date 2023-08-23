use std::{error::Error, marker::PhantomData, vec::IntoIter};

use crate::pattern::InnerIterable;

use super::{model_builder_type::ModelBuilderType, TrainingConfig};

pub struct ModelBuilder<Mod, Dat, SomeInnerIterable, E, Pattern>
where
    E: Error,
    Pattern: Clone + Ord + 'static + Default,
    SomeInnerIterable: InnerIterable<Pattern, IntoIter<Pattern>>,
    Dat: Iterator<Item = Result<SomeInnerIterable, E>>,
{
    pub type_of: ModelBuilderType,
    pub config: TrainingConfig,
    pub model: Mod,
    pub data: Dat,
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
        let model = ThreadSafeModel::<'_, '_, String, ()>::new();

        assert!(!model.model.is_poisoned())
    }
}
