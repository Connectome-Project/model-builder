use petgraph::stable_graph::IndexType;

pub use self::node::Node;
pub use self::node_type::NodeType;
use crate::arc_model::model::Model;

use crate::pattern::PatternTrait;
use std::{
    fmt::Debug,
    fmt::Display,
    sync::{Arc, RwLock},
};

pub mod connection;
pub mod graph_change_request;
pub mod model;
mod model_trait;
mod node;
mod node_type;

pub struct ThreadSafeModel<T, R, Ix>
where
    T: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
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
    Ix: IndexType + Clone,
{
    pub data: Arc<RwLock<Model<T, R, Ix>>>,
}

impl<T, R, Ix> ThreadSafeModel<T, R, Ix>
where
    T: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
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
    Ix: IndexType + Clone,
{
    pub fn new() -> Self {
        ThreadSafeModel {
            data: Arc::new(RwLock::new(Model::<T, R, Ix>::new())),
        }
    }
}

impl<T, R, Ix> Clone for ThreadSafeModel<T, R, Ix>
where
    T: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
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
    Ix: IndexType + Clone,
{
    fn clone(&self) -> Self {
        ThreadSafeModel {
            data: self.data.clone(),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::{Arc, RwLock};

    use super::{model::Model, ThreadSafeModel};

    #[test]
    fn create_model() {
        let arc = ThreadSafeModel {
            data: Arc::new(RwLock::new(Model::<String, String, usize>::new())),
        };
        let read_lock = arc.data.read();
        if let Ok(lock) = read_lock {
            // assert_eq!(lock.);
        }
    }
}
