use std::default::Default;
use std::fmt::Debug;

pub trait Pattern: Debug + Clone + ToString + Default + Sized + From<String> {
    fn concat(&self, rhs: Self) -> Self;
}

impl Pattern for String {
    fn concat(&self, rhs: Self) -> Self {
        self.to_owned() + &rhs.clone()
    }
}
