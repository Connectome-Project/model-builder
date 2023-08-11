use std::str::Chars;

pub trait InputTwoWayIterable<R>
where
    R: Iterator,
{
    fn get_symbol_iterator_at(&self, index: usize) -> Option<R>;
}

pub struct DataContainer<'a, T>
where
    T: 'a + IntoIterator,
{
    data: &'a T,
}

impl<'a, T> DataContainer<'a, T>
where
    T: 'a + IntoIterator,
{
    #[allow(dead_code)]
    pub fn new(data: &'a T) -> Self {
        DataContainer { data }
    }

    #[allow(dead_code)]
    pub fn get_data(&self) -> &'a T {
        self.data
    }
}

impl<'a> InputTwoWayIterable<Chars<'a>> for DataContainer<'a, Vec<&'a str>> {
    fn get_symbol_iterator_at(&self, index: usize) -> Option<Chars<'a>> {
        let double_ref = self.data.iter().nth(index);
        match double_ref {
            Some(x) => Some((*x).chars()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_segment_can_be_created() {
        let input = vec!["This ist the first line.", "This is the second line."];

        let data_segment = DataContainer::new(&input);
        assert_eq!(data_segment.get_data().len(), 2);
    }

    #[test]
    fn test_lines_from_file() {
        let file_lines = vec!["Line 1", "Line 2", "Line 3"];
        let container = DataContainer::new(&file_lines);

        let mut lines_iter = container.get_symbol_iterator_at(1).unwrap();
        assert_eq!(lines_iter.next(), Some('L'));
        assert_eq!(lines_iter.next(), Some('i'));
        assert_eq!(lines_iter.next(), Some('n'));
        assert_eq!(lines_iter.next(), Some('e'));
        assert_eq!(lines_iter.next(), Some(' '));
        assert_eq!(lines_iter.next(), Some('2'));
        assert_eq!(lines_iter.next(), None);
    }
}
