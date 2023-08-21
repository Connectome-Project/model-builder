use std::{fmt::Display, iter::Peekable, vec::IntoIter};

use regex::Regex;

use crate::arc_model::Node;

use super::PatternTrait;

#[derive(PartialEq, Eq, Debug)]
pub struct LongestPattern<'a, Pattern>
where
    Pattern: Clone + Ord + 'static + PatternTrait + Display + Default,
{
    pub matching_node: &'a Node<Pattern>,
    pub pattern_so_far: Pattern,
}

pub fn find_longest_pattern<'a, PatternContent>(
    nodes: Vec<&'a Node<PatternContent>>,
    mut matched_node: Option<&'a Node<PatternContent>>,
    mut data_iterator: Peekable<IntoIter<PatternContent>>,
    mut pattern_so_far: PatternContent,
) -> Option<LongestPattern<'a, PatternContent>>
where
    PatternContent: Clone + Ord + 'static + PatternTrait + Display + Default,
{
    if let Some(_) = data_iterator.peek() {
        let extended_pattern_so_far = pattern_so_far.concat(&data_iterator.next().unwrap());
        let mut starts_with: Vec<&Node<PatternContent>> = nodes
            .iter()
            .map(|f| *f)
            .filter(|node| {
                return node.pattern.starts_with(&extended_pattern_so_far);
            })
            .collect::<Vec<&Node<PatternContent>>>();

        starts_with.sort_by(|a, b| b.cmp(a));

        let mut matched_nodes = starts_with
            .iter()
            .map(|f| *f)
            .filter(|node| {
                let reg = Regex::new(&format!(r"{}", node.pattern)).unwrap();
                return extended_pattern_so_far.match_against(reg);
            })
            .collect::<Vec<&Node<PatternContent>>>();

        matched_nodes.sort_by(|a, b| b.cmp(a));

        let longest_matched_node = match matched_nodes.first() {
            Some(c) => Some(*c),
            None => None,
        };

        match starts_with.get(0) {
            Some(first_node) => {
                if first_node.pattern.len() > extended_pattern_so_far.len() {
                    return find_longest_pattern(
                        starts_with,
                        longest_matched_node,
                        data_iterator,
                        extended_pattern_so_far,
                    );
                } else {
                    if longest_matched_node.is_some() {
                        return Some(LongestPattern {
                            matching_node: longest_matched_node.unwrap(),
                            pattern_so_far: extended_pattern_so_far.clone(),
                        });
                    }
                    if matched_node.is_some() {
                        return Some(LongestPattern {
                            matching_node: matched_node.unwrap(),
                            pattern_so_far: pattern_so_far.clone(),
                        });
                    }
                }
            }
            None => {}
        }
    }
    if matched_node.is_some() && pattern_so_far != PatternContent::default() {
        return Some(LongestPattern {
            matching_node: matched_node.unwrap(),
            pattern_so_far: pattern_so_far.clone(),
        });
    }
    return None;
}

#[cfg(test)]
mod tests {
    use crate::{
        arc_model::{Node, NodeType},
        pattern::{find_longest_pattern, LongestPattern},
    };

    #[test]
    fn test_find_longest_pattern() {
        let node1 = Node::new("h".to_string(), NodeType::Start);
        let node2 = Node::new("h.".to_string(), NodeType::Start);
        let node3 = Node::new("l".to_string(), NodeType::Start);

        let nodes = vec![&node1, &node2, &node3];
        let nodes_cloned = nodes.clone();
        let data = "hello"
            .to_string()
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .into_iter();

        let res: LongestPattern<'_, String> =
            find_longest_pattern(nodes, None, data.peekable(), "".to_string()).unwrap();
        println!("{:?}", res);
        assert_eq!(nodes_cloned.get(1).unwrap(), &res.matching_node);
        assert_eq!("he", res.pattern_so_far);
    }

    #[test]
    fn test_find_long_pattern_with_wildcard() {
        let node2 = Node::new("he.l".to_string(), NodeType::Start);
        let node3 = Node::new("l".to_string(), NodeType::Start);

        let nodes = vec![&node2, &node3];
        let nodes_cloned = nodes.clone();
        let data = "hello"
            .to_string()
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .into_iter();

        let res: LongestPattern<'_, String> =
            find_longest_pattern(nodes, None, data.peekable(), "".to_string()).unwrap();
        println!("{:?}", res);
        assert_eq!(nodes_cloned.get(0).unwrap(), &res.matching_node);
        assert_eq!("hell", res.pattern_so_far);
    }
}
