use std::vec::IntoIter;

pub trait InnerIterable<It: Clone + Ord + 'static, Iter: Iterator<Item = It>>:
    FromIterator<It>
{
    fn get_inner_iterable(&self) -> Iter;
}

impl InnerIterable<String, IntoIter<String>> for String {
    fn get_inner_iterable(&self) -> IntoIter<String> {
        let df = self.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        let iter: std::vec::IntoIter<String> = df.into_iter();
        return iter;
    }
}
