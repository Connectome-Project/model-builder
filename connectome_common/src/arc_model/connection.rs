use std::fmt::{Debug, Display};

use crate::pattern::PatternTrait;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Connection<R>
where
    R: Clone
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Display
        + Debug
        + Default
        + PatternTrait
        + 'static,
{
    pub connection_info: Option<Vec<ConnectionInfo<R>>>,
}

impl<R> Connection<R>
where
    R: Clone
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Display
        + Debug
        + Default
        + PatternTrait
        + 'static,
{
    pub fn build_from_content(info: ConnectionInfo<R>) -> Self {
        Connection {
            connection_info: Some(vec![info]),
        }
    }

    #[allow(dead_code)]
    fn add_content(&mut self, info: ConnectionInfo<R>) -> () {
        if let None = self.connection_info {
            self.connection_info = Some(vec![]);
        }

        self.connection_info.as_mut().unwrap().push(info);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ConnectionInfo<Pattern>
where
    Pattern: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
{
    pub label: Pattern,
}

impl<Pattern> Display for ConnectionInfo<Pattern>
where
    Pattern: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}
