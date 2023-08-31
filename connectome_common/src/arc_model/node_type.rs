#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Eq, PartialOrd, Ord)]
pub enum NodeType {
    Start,
    Generated,
    End,
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Start
    }
}
