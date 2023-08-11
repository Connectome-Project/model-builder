use super::node::Node;

#[allow(dead_code)]
pub struct ConnectionToNode<'a, T, R>
where
    T: Clone + PartialEq + 'static + Ord,
{
    pub node: &'a Node<T>,
    pub connection_info: Option<R>,
}
