use self::model::ModelTrait;
use std::sync::{Arc, RwLock};

mod model;
mod node_type;

type ModelTraitObject<'a, 'b, T, R> = Box<dyn ModelTrait<'a, 'b, T, R>>;

struct ThreadSafeModel<'a, 'b, T, R>
where
    T: Clone + 'static,
    R: 'static,
{
    model: Arc<RwLock<ModelTraitObject<'a, 'b, T, R>>>,
}

trait ThreadSafeModelTrait<'a, 'b, T: Clone + 'static, R: 'static> {
    fn new<F>(func: F) -> Self
    where
        F: FnOnce() -> ModelTraitObject<'a, 'b, T, R>;
}

impl<'a, 'b, T, R> ThreadSafeModelTrait<'a, 'b, T, R> for ThreadSafeModel<'a, 'b, T, R>
where
    T: Clone + 'static,
    R: 'a + 'static,
{
    fn new<F>(func: F) -> Self
    where
        F: FnOnce() -> ModelTraitObject<'a, 'b, T, R>,
    {
        ThreadSafeModel {
            model: Arc::new(RwLock::new(func())),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::arc_model::model::Node;

    use super::{model::Model, *};

    #[test]
    fn create_model() {
        let model = ThreadSafeModel::new(|| Box::new(Model::<String, ()>::new()));
        let read_lock = model.model.read();
        if let Ok(lock) = read_lock {
            let nodes = lock.get_nodes();
            assert_eq!(nodes.len(), 0);
        }
    }

    #[test]
    fn can_add_node() {
        let model = ThreadSafeModel::new(|| Box::new(Model::<String, ()>::new()));
        let write_lock = model.model.write();
        if let Ok(mut lock) = write_lock {
            let nodes: &mut Vec<Node<String>> = lock.get_mut_nodes();
            let new_node = Node::new(String::from("hello"), node_type::NodeType::Start);
            let new_node_copy = new_node.clone();
            nodes.push(new_node);
            let first_node = nodes.get(0);

            assert_eq!(nodes.len(), 1);
            assert_eq!(first_node, Some(&new_node_copy))
        }
    }

    #[test]
    fn can_find_node() {
        let model = ThreadSafeModel::new(|| Box::new(Model::<String, ()>::new()));
        let write_lock = model.model.write();
        if let Ok(mut lock) = write_lock {
            let nodes: &mut Vec<Node<String>> = lock.get_mut_nodes();
            let new_node = Node::new(String::from("hello"), node_type::NodeType::Start);
            let new_node_copy = new_node.clone();
            nodes.push(new_node);
            let first_node = nodes.get(0);

            assert_eq!(nodes.len(), 1);
            assert_eq!(first_node, Some(&new_node_copy))
        }
    }

    #[test]
    fn can_get_node_connection() {
        let model = ThreadSafeModel::new(|| Box::new(Model::<String, ()>::new()));
        let write_lock = model.model.write();
        if let Ok(mut lock) = write_lock {
            let nodes: &mut Vec<Node<String>> = lock.get_mut_nodes();
            let new_node = Node::new(String::from("hello"), node_type::NodeType::Start);
            let new_node_copy = new_node.clone();
            nodes.push(new_node);
            let first_node = nodes.get(0);

            assert_eq!(nodes.len(), 1);
            assert_eq!(first_node, Some(&new_node_copy))
        }
    }
}
