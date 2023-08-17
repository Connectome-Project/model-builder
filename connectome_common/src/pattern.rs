use regex::Regex;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Pattern<ContentType>
where
    ContentType: Clone + PartialEq + Ord + Default,
{
    content: ContentType,
}

pub trait PatternTrait
where
    Self: Sized,
{
    fn concat(&self, rhs: &Self) -> Self;
    fn len(&self) -> usize;
    fn match_against(&self, regex: Regex) -> bool;
}

impl PatternTrait for String {
    fn concat(&self, rhs: &Self) -> Self {
        self.clone() + rhs
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn match_against(&self, regex: Regex) -> bool {
        regex.is_match(&self)
    }
}
