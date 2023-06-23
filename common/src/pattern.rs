use regex::Regex;

pub mod string_pattern;

// pub trait PatternContent: Clone + Default {}

#[derive(Debug, Clone, Default)]
pub struct Pattern<ContentType>
where
    ContentType: Clone + Default,
{
    content: ContentType,
}

pub trait PatternTrait {
    type ContentType: Clone + Default;

    fn get_pattern(&self) -> &Self::ContentType;
    fn concat(&self, rhs: &Self) -> Self;
    fn len(&self) -> usize;
    fn match_against(&self, regex: Regex) -> bool;
}

impl<ContentType: Clone + Default + Sized> Pattern<ContentType> {
    #[allow(dead_code)]
    pub fn new() -> Pattern<ContentType> {
        Pattern::<ContentType> {
            content: ContentType::default(),
        }
    }

    #[allow(dead_code)]
    pub fn build(default: &ContentType) -> Pattern<ContentType> {
        Pattern::<ContentType> {
            content: default.clone(),
        }
    }
}
