use std::fmt::{Debug, Display};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Connection<R = ConnectionInfo>
where
    R: Clone + PartialEq + Eq + Ord + PartialOrd + Display + Debug + Default,
{
    pub connection_info: Option<Vec<R>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ConnectionInfo {
    label: String,
}

impl Display for ConnectionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}
