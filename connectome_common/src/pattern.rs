mod inner_iterable;
mod longest_path;

pub use inner_iterable::InnerIterable;
pub use longest_path::{find_longest_pattern, LongestPattern};
use regex::Regex;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Pattern<ContentType>
where
    ContentType: Clone + PartialEq + Ord + Default,
{
    content: ContentType,
}

pub trait PatternTrait
where
    Self: Sized,
{
    fn concat(&self, rhs: &Self) -> Self;
    fn len(&self) -> usize;
    fn match_against(&self, regex: Regex) -> bool; // has to be inverted node must match pattern so it should be the trait of a node
    fn starts_with(&self, data_that_starts_with: &Self) -> bool;
}

impl PatternTrait for String {
    fn concat(&self, rhs: &Self) -> Self {
        self.clone() + rhs
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn match_against(&self, regex: Regex) -> bool {
        regex.is_match(&self)
    }
    fn starts_with(&self, pattern_for_regex: &Self) -> bool {
        let pattern_length = pattern_for_regex.len();
        if self.len() >= pattern_length {
            let self_content = get_first_n_chars(self, pattern_length);
            // println!("{} starts_with: {}", self, pattern_for_regex);
            let regex = Regex::new(format!(r"{}", self_content).as_str()).unwrap();
            return regex.is_match(&pattern_for_regex);
        }

        false
    }
}

fn get_first_n_chars(input: &str, n: usize) -> &str {
    let mut char_indices = input.char_indices();

    for _ in 0..n {
        if let Some((idx, _)) = char_indices.next() {
            // idx is the byte index of the character's start in the input string
        } else {
            // If there are fewer than n characters in the input string
            return input;
        }
    }

    &input[..char_indices
        .next()
        .map(|(idx, _)| idx)
        .unwrap_or(input.len())]
}
