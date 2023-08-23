use crate::pattern::PatternTrait;

use self::model::Model;
pub use self::node::Node;
pub use self::node_type::NodeType;
use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};

pub mod connection;
mod model;
mod node;
mod node_type;

pub struct ThreadSafeModel<'a, 'b, T, R>
where
    T: Clone + Ord + 'static + PatternTrait + Display + Default,
    R: 'static,
{
    pub model: Arc<RwLock<Model<'a, 'b, T, R>>>,
}

impl<'a, 'b, T, R> ThreadSafeModel<'a, 'b, T, R>
where
    T: Clone + Ord + 'static + PatternTrait + Display + Default,
    R: 'static,
{
    pub fn new() -> Self {
        ThreadSafeModel {
            model: Arc::new(RwLock::new(Model::new())),
        }
    }
}

#[cfg(test)]
mod tests {

    // use crate::arc_model::model::Node;

    // use super::{model::Model, *};

    use std::sync::{Arc, RwLock};

    use super::{model::Model, ThreadSafeModel};

    #[test]
    fn create_model() {
        let arc = ThreadSafeModel {
            model: Arc::new(RwLock::new(Model::<'_, '_, String, ()>::new())),
        };
        let read_lock = arc.model.read();
        if let Ok(lock) = read_lock {
            let nodes = lock.get_nodes();
            assert_eq!(nodes.len(), 0);
        }
    }
}
