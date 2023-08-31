use std::{
    fmt::{Debug, Display},
    iter::Peekable,
    option::Option,
    vec::IntoIter,
};

use petgraph::stable_graph::NodeIndex;
use regex::Regex;

use crate::arc_model::Node;

use super::PatternTrait;

pub enum LongestPatternResult<'a, P, Ix>
where
    P: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Self: Display,
    Ix: Clone + Debug,
{
    ResultWithIter(LongestPattern<'a, P, Ix>),
    Iter(Peekable<IntoIter<P>>),
}

impl<'a, P, Ix> LongestPatternResult<'a, P, Ix>
where
    P: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Self: Display,
    Ix: Clone + Debug,
{
    pub fn is_some(&self) -> bool {
        if let LongestPatternResult::ResultWithIter(d) = self {
            return true;
        }
        false
    }

    pub fn is_none(&self) -> bool {
        if let LongestPatternResult::Iter(d) = self {
            return true;
        }
        false
    }
}

impl<'a, Pattern, Ix> std::fmt::Display for LongestPatternResult<'a, Pattern, Ix>
where
    Pattern: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Ix: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match &self {
            LongestPatternResult::ResultWithIter(d) => write!(f, "{:?}", d),
            LongestPatternResult::Iter(i) => write!(f, "{:?}", i),
        };
    }
}

#[derive(Debug)]
pub struct LongestPattern<'a, Pattern, Ix>
where
    Pattern: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Ix: Clone + Debug,
{
    pub matching_node: NodeWithOptionalIdx<'a, Pattern, Ix>,
    pub pattern_so_far: Pattern,
    pub remaining_iter: Peekable<IntoIter<Pattern>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CloneableOption<T: Clone>(Option<T>);

impl<T: Clone> CloneableOption<T> {
    pub fn new_some(t: T) -> Self {
        CloneableOption(Some(t))
    }
    pub fn new_none() -> Self {
        CloneableOption(None)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeWithOptionalIdx<'a, PatternContent, Ix: Clone + Debug>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
{
    pub node: &'a Node<PatternContent>,
    pub index: CloneableOption<NodeIndex<Ix>>,
}

pub fn find_longest_pattern<'a, PatternContent, Ix>(
    nodes: Vec<NodeWithOptionalIdx<'a, PatternContent, Ix>>,
    matched_node: Option<NodeWithOptionalIdx<'a, PatternContent, Ix>>,
    mut data_iterator: Peekable<IntoIter<PatternContent>>,
    pattern_so_far: PatternContent,
) -> LongestPatternResult<'a, PatternContent, Ix>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Ix: Clone + Debug,
{
    if let Some(_) = data_iterator.peek() {
        let extended_pattern_so_far = pattern_so_far.concat(&data_iterator.next().unwrap());

        //subpattern match for equal length
        let mut starts_with_subpattern_match: Vec<NodeWithOptionalIdx<'_, PatternContent, Ix>> =
            filter_nodes(nodes, |node_info| {
                return node_info.node.pattern.starts_with(&extended_pattern_so_far);
            });

        starts_with_subpattern_match.sort_by(|a, b| b.node.cmp(a.node));

        //proper (full) match
        let mut matched_nodes = filter_nodes(starts_with_subpattern_match.clone(), |node_info| {
            let reg = Regex::new(&format!(r"{}", node_info.node.pattern)).unwrap();
            return extended_pattern_so_far.match_against(reg);
        });

        matched_nodes.sort_by(|a, b| b.node.cmp(a.node));

        let longest_matched_node: Option<NodeWithOptionalIdx<'_, PatternContent, Ix>> =
            match matched_nodes.first() {
                Some(c) => Some(c.clone()),
                None => None,
            };

        match starts_with_subpattern_match.get(0) {
            Some(first_node) => {
                // we have a possible solution with bigger length, worth investigating
                if first_node.node.pattern.len() > extended_pattern_so_far.len() {
                    return find_longest_pattern(
                        starts_with_subpattern_match,
                        longest_matched_node,
                        data_iterator,
                        extended_pattern_so_far,
                    );
                } else {
                    // the currently found is the longest
                    if longest_matched_node.is_some() {
                        return LongestPatternResult::ResultWithIter(LongestPattern::<
                            'a,
                            PatternContent,
                            Ix,
                        > {
                            matching_node: longest_matched_node.unwrap(),
                            pattern_so_far: extended_pattern_so_far.clone(),
                            remaining_iter: data_iterator,
                        });
                    }
                }
            }
            None => {}
        }
    }
    // previous found is the longest
    if matched_node.is_some() && pattern_so_far != PatternContent::default() {
        return LongestPatternResult::ResultWithIter(LongestPattern {
            matching_node: matched_node.unwrap(),
            pattern_so_far: pattern_so_far.clone(),
            remaining_iter: data_iterator,
        });
    }
    //empty pattern, empty nodes
    return LongestPatternResult::Iter(data_iterator);
}

fn filter_nodes<'a, PatternContent, Idx>(
    nodes_with_indices: Vec<NodeWithOptionalIdx<'a, PatternContent, Idx>>,
    predicate: impl FnMut(&NodeWithOptionalIdx<'a, PatternContent, Idx>) -> bool,
) -> Vec<NodeWithOptionalIdx<'a, PatternContent, Idx>>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default + Debug,
    Idx: Clone + Debug,
{
    nodes_with_indices
        .into_iter()
        .filter(predicate)
        .collect::<Vec<NodeWithOptionalIdx<'a, PatternContent, Idx>>>()
}

#[cfg(test)]
mod tests {
    use petgraph::stable_graph::NodeIndex;

    use crate::{
        arc_model::{Node, NodeType},
        pattern::{find_longest_pattern, LongestPattern, LongestPatternResult},
    };

    use super::{CloneableOption, NodeWithOptionalIdx};

    fn create_input(s: &str) -> std::vec::IntoIter<String> {
        let data = s
            .to_string()
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .into_iter();
        data
    }

    fn create_nodes_with_optional_idx(
        input: &Vec<Node<String>>,
    ) -> Vec<NodeWithOptionalIdx<'_, String, usize>> {
        let mut result = Vec::new();
        for (idx, node) in input.iter().enumerate() {
            let node_with_idx = NodeWithOptionalIdx {
                node: &node,
                index: CloneableOption::new_some(NodeIndex::new(idx)),
            };
            result.push(node_with_idx);
        }
        result
    }

    fn create_nodes(str_slice: Vec<String>) -> Vec<Node<String>> {
        let mut result: Vec<Node<String>> = Vec::new();

        for str in str_slice.into_iter() {
            let node = Node::new(str, NodeType::Generated);
            result.push(node);
        }

        result
    }

    #[test]
    fn test_find_longest_pattern_with_different_length_patterns() {
        let nodes = create_nodes(vec!["h".to_string(), "h.".to_string(), "l".to_string()]);
        let nodes_with_idx = create_nodes_with_optional_idx(&nodes);
        let nodes_with_idx_cloned = nodes_with_idx.clone();
        let data = create_input("hello");

        let res = find_longest_pattern(nodes_with_idx, None, data.peekable(), "".to_string());

        if let LongestPatternResult::ResultWithIter(data) = res {
            assert_eq!(nodes_with_idx_cloned.get(1).unwrap(), &data.matching_node);
            assert_eq!("he", data.pattern_so_far);
        } else {
            panic!("fail");
        }
    }

    #[test]
    fn test_find_long_pattern_with_wildcard() {
        let nodes = create_nodes(vec!["he.l".to_string(), "l.".to_string()]);
        let nodes_with_idx = create_nodes_with_optional_idx(&nodes);
        let nodes_cloned = nodes_with_idx.clone();
        let data = create_input("hello");

        let res = find_longest_pattern(nodes_with_idx, None, data.peekable(), "".to_string());

        if let LongestPatternResult::ResultWithIter(result) = res {
            let remaining_collected_iter = result.remaining_iter.collect::<String>();
            assert_eq!(nodes_cloned.get(0).unwrap(), &result.matching_node);
            assert_eq!("hell", result.pattern_so_far);
            assert_eq!("o", remaining_collected_iter);
        } else {
            panic!("failure")
        }
    }

    #[test]
    fn test_find_long_pattern_finds_longest() {
        let nodes = create_nodes(vec!["he".to_string(), "h".to_string()]);
        let nodes_with_idx = create_nodes_with_optional_idx(&nodes);
        let nodes_cloned = nodes_with_idx.clone();
        let data = create_input("hello");

        let res = find_longest_pattern(nodes_with_idx, None, data.peekable(), "".to_string());
        if let LongestPatternResult::ResultWithIter(result) = res {
            let remaining_collected_iter = result.remaining_iter.collect::<String>();
            assert_eq!(nodes_cloned.get(0).unwrap(), &result.matching_node);
            assert_eq!("he", result.pattern_so_far);
            assert_eq!("llo", remaining_collected_iter);
        } else {
            panic!("fail")
        }
    }

    #[test]
    fn test_find_long_pattern_with_wildly_different_length_patterns() {
        let nodes = create_nodes(vec!["h".to_string(), "he.l".to_string(), "l".to_string()]);
        let nodes_with_idx = create_nodes_with_optional_idx(&nodes);
        let nodes_cloned = nodes_with_idx.clone();
        let data = create_input("hello");

        let res = find_longest_pattern(nodes_with_idx, None, data.peekable(), "".to_string());
        if let LongestPatternResult::ResultWithIter(result) = res {
            let remaining_collected_iter = result.remaining_iter.collect::<String>();
            assert_eq!(nodes_cloned.get(1).unwrap(), &result.matching_node);
            assert_eq!("hell", result.pattern_so_far);
            assert_eq!("o", remaining_collected_iter);
        } else {
            panic!("failure")
        }
    }

    #[test]
    fn test_find_long_pattern_no_node_match() {
        let nodes = create_nodes(vec!["l".to_string()]);
        let nodes_with_idx = create_nodes_with_optional_idx(&nodes);
        let data = create_input("hello");

        let res = find_longest_pattern(nodes_with_idx, None, data.peekable(), "".to_string());
        assert!(res.is_none());
    }

    #[test]
    fn test_find_long_pattern_data_is_empty() {
        let nodes = create_nodes(vec!["h".to_string()]);
        let nodes_with_idx = create_nodes_with_optional_idx(&nodes);

        let res = find_longest_pattern(
            nodes_with_idx,
            None,
            vec!["".to_string()].into_iter().peekable(),
            "".to_string(),
        );
        assert!(res.is_none());
    }
}
