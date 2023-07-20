use std::collections::BTreeMap;

use super::node_type::NodeType;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Eq, PartialOrd, Ord)]
pub struct Node<T>
where
    T: Clone + 'static + PartialEq + Ord,
{
    pattern: T,
    node_type: NodeType,
}

impl<T> Node<T>
where
    T: Clone + PartialEq + 'static + Ord,
{
    pub fn new(pattern: T, node_type: NodeType) -> Self {
        Node { pattern, node_type }
    }
}

#[allow(dead_code)]
pub struct ConnectionToNode<'a, T, R>
where
    T: Clone + PartialEq + 'static + Ord,
{
    pub node: &'a Node<T>,
    pub connection_info: Option<R>,
}

pub struct Model<'a, 'b, T, R>
where
    T: Clone + PartialEq + 'static + Ord,
    R: 'static,
{
    nodes: Vec<Node<T>>,
    #[allow(dead_code)]
    connections_from: BTreeMap<&'b Node<T>, Vec<ConnectionToNode<'a, T, R>>>,
}

impl<'a, 'b, T, R> Model<'a, 'b, T, R>
where
    T: Clone + PartialEq + 'static + Ord,
    R: 'static,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Model {
            nodes: Vec::new(),
            connections_from: BTreeMap::new(),
        }
    }
}

pub trait ModelTrait<'a, 'b, T, R>
where
    T: Clone + PartialEq + 'static + Ord,
    R: 'static,
{
    fn get_nodes(&self) -> &Vec<Node<T>>;
    fn get_mut_nodes(&mut self) -> &mut Vec<Node<T>>;
    fn find_connection_from_node(&self, node: &Node<T>) -> Vec<&ConnectionToNode<T, R>>;
    fn add_connection(
        &mut self,
        from_node: &'b Node<T>,
        to_node: &'a Node<T>,
        connection_info: Option<R>,
    );
}

impl<'a, 'b, T, R> ModelTrait<'a, 'b, T, R> for Model<'a, 'b, T, R>
where
    T: Clone + PartialEq + Ord,
{
    fn get_nodes(&self) -> &Vec<Node<T>> {
        &self.nodes
    }

    fn get_mut_nodes(&mut self) -> &mut Vec<Node<T>> {
        &mut self.nodes
    }

    fn find_connection_from_node(&self, node: &Node<T>) -> Vec<&ConnectionToNode<T, R>> {
        let res: Vec<&ConnectionToNode<T, R>> = self
            .connections_from
            .iter()
            .filter(|d| d.0 == &node)
            .flat_map(|d| d.1)
            .collect();
        return res;
    }

    fn add_connection(
        &mut self,
        from_node: &'b Node<T>,
        to_node: &'a Node<T>,
        connection_info: Option<R>,
    ) {
        let from_node_vec_found = self.connections_from.get_mut(from_node);
        if let Some(node_vec) = from_node_vec_found {
            let found_to = node_vec.iter().find(|e| e.node == to_node);
            if found_to.is_none() {
                let conn = ConnectionToNode {
                    connection_info: connection_info,
                    node: to_node,
                };
                node_vec.push(conn);
            }
        } else {
            let added_connection = ConnectionToNode {
                connection_info: connection_info,
                node: to_node,
            };

            let connection_vector = vec![added_connection];
            self.connections_from.insert(from_node, connection_vector);
        }
    }
}
