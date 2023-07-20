#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Eq, PartialOrd, Ord)]
pub enum NodeType {
    Start,
    Regular,
    Generated,
    End,
}
