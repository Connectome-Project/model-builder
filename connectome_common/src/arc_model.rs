use self::model::ModelTrait;
use std::error::Error;
use std::sync::{Arc, RwLock, RwLockReadGuard};

mod model;
mod node_type;

type ModelTraitObject<T, R> = Box<dyn ModelTrait<T, R>>;

struct ThreadSafeModel<T, R>
where
    T: Clone + 'static,
    R: 'static,
{
    model: Arc<RwLock<Box<dyn ModelTrait<T, R>>>>,
}

trait ThreadSafeModelTrait<T: Clone + 'static, R: 'static> {
    fn new<F>(func: F) -> Self
    where
        F: FnOnce() -> Box<dyn ModelTrait<T, R>>;

    fn get_read_model<'a>(
        &'a self,
    ) -> Result<RwLockReadGuard<ModelTraitObject<T, R>>, Box<dyn Error + 'a>>;
}

impl<'a, T, R> ThreadSafeModelTrait<T, R> for ThreadSafeModel<T, R>
where
    T: Clone + 'static,
    R: 'a + 'static,
{
    fn new<F>(func: F) -> Self
    where
        F: FnOnce() -> Box<dyn ModelTrait<T, R>>,
    {
        ThreadSafeModel {
            model: Arc::new(RwLock::new(func())),
        }
    }

    fn get_read_model<'b>(
        &'b self,
    ) -> Result<RwLockReadGuard<ModelTraitObject<T, R>>, Box<dyn Error + 'b>> {
        let lock = self.model.read()?;
        Result::Ok(lock)
    }
}

#[cfg(test)]
mod tests {

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
}
