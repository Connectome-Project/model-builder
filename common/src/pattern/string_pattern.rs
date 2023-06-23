use super::{Pattern, PatternTrait};
use regex::Regex;

impl PatternTrait for Pattern<String> {
    type ContentType = String;

    fn get_pattern(&self) -> &Self::ContentType {
        &self.content
    }

    fn concat(&self, rhs: &Self) -> Self {
        Pattern {
            content: self.content.clone() + &rhs.content,
        }
    }

    fn len(&self) -> usize {
        self.content.len()
    }

    fn match_against(&self, regex: Regex) -> bool {
        regex.is_match(&self.content)
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::Pattern;

    #[test]
    fn pattern_can_be_created_with_empty_value() {
        let pattern = Pattern::<String>::new();
        assert_eq!(pattern.content, "");
    }

    #[test]
    fn pattern_can_be_created_wit_default_value() {
        let pattern = Pattern::<String>::build(&"default".to_string());
        assert_eq!(pattern.content, "default".to_string());
    }

    mod string_pattern {
        use regex::Regex;

        use crate::pattern::PatternTrait;

        use super::*;

        #[test]
        fn can_get_pattern() {
            let pattern = Pattern::<String>::build(&"default".to_string());
            assert_eq!(pattern.get_pattern(), &"default".to_string());
        }

        #[test]
        fn concatenates_two_patterns() {
            let pattern = Pattern::<String>::build(&"default".to_string());
            let second_pattern = Pattern::<String>::build(&"sec".to_string());
            let result: Pattern<String> = pattern.concat(&second_pattern);
            assert_eq!(result.get_pattern(), &"defaultsec".to_string());
        }

        #[test]
        fn gives_length_properly() {
            let pattern = Pattern::<String>::build(&"default".to_string());
            let result: usize = pattern.len();
            assert_eq!(result, 7);
        }

        #[test]
        fn can_match_against_regex() {
            let pattern = Pattern::<String>::build(&"default".to_string());
            let regex: Regex = Regex::new(r"def").unwrap();
            let result: bool = pattern.match_against(regex);
            assert_eq!(result, true);
        }
    }
}
