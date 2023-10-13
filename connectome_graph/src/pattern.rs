use std::default::Default;
use std::fmt::Debug;

// #[derive(Clone, Debug)]
// pub struct PatternContainer<T: Pattern> {
//     pub data: T,
// }

// impl<T: Pattern> From<T> for PatternContainer<T> {
//     fn from(value: T) -> Self {
//         PatternContainer { data: value }
//     }
// }

// impl<T: Pattern> Default for PatternContainer<T> {
//     fn default() -> Self {
//         Self {
//             data: Default::default(),
//         }
//     }
// }

pub trait Pattern: Debug + Clone + ToString + Default + Sized + From<String> {
    fn concat(&self, rhs: Self) -> Self;
}

impl Pattern for String {
    fn concat(&self, rhs: Self) -> Self {
        self.to_owned() + &rhs.clone()
    }
}
