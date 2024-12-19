#![allow(dead_code)]

use std::fs;

#[derive(Debug)]
struct PrefixTrieNode {
    c: u8,
    end_of_word: bool,
    children: Vec<PrefixTrieNode>,
}
impl PrefixTrieNode {
    fn new(c: u8) -> Self {
        PrefixTrieNode {
            c,
            end_of_word: false,
            children: Vec::new(),
        }
    }

    fn insert(&mut self, word: &[u8]) {
        if !word.is_empty() {
            if let Some(child) = self.children.iter_mut().find(|child| child.c == word[0]) {
                child.insert(&word[1..]);
            } else {
                let mut new_child = PrefixTrieNode::new(word[0]);
                new_child.insert(&word[1..]);
                self.children.push(new_child);
            }
        } else {
            self.end_of_word = true;
        }
    }

    fn matching_substr_lengths(&self, word: &[u8]) -> Vec<usize> {
        let mut node = self;
        let mut i = 0;
        let mut matching = Vec::new();
        while i < word.len() {
            if let Some(child) = node.children.iter().find(|child| child.c == word[i]) {
                node = child;
                i += 1;
                if node.end_of_word {
                    matching.push(i);
                }
            } else {
                break;
            }
        }
        matching
    }
}

fn parse_input(filename: &str) -> (PrefixTrieNode, String) {
    let contents = fs::read_to_string("input/2024/19/".to_owned() + filename).unwrap();
    let mut parts = contents.split("\n\n");
    let patterns_str = parts.next().unwrap();

    let mut prefix_trie = PrefixTrieNode::new(0);
    for pattern in patterns_str.split(", ") {
        prefix_trie.insert(pattern.as_bytes());
    }

    let designs = parts.next().unwrap().to_string();

    (prefix_trie, designs)
}

fn ways_design_is_producible(prefix_trie: &PrefixTrieNode, design: &[u8]) -> usize {
    let mut visited = vec![0usize; design.len() + 1];
    visited[0] = 1;
    for i in 0..visited.len() {
        if visited[i] == 0 {
            continue;
        }
        for len in prefix_trie.matching_substr_lengths(&design[i..]) {
            visited[i + len] += visited[i];
        }
    }
    visited[design.len()]
}

fn count_producible_designs(filename: &str) -> usize {
    let (prefix_trie, design_str) = parse_input(filename);
    design_str
        .split_whitespace()
        .filter(|design| ways_design_is_producible(&prefix_trie, design.as_bytes()) > 0)
        .count()
}

fn sum_ways_designs_are_producible(filename: &str) -> usize {
    let (prefix_trie, design_str) = parse_input(filename);
    design_str
        .split_whitespace()
        .map(|design| ways_design_is_producible(&prefix_trie, design.as_bytes()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{count_producible_designs, sum_ways_designs_are_producible};

    #[test]
    fn part1_example() {
        let result = count_producible_designs("example.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn part1() {
        let result = count_producible_designs("input.txt");
        assert_eq!(result, 206);
    }

    #[test]
    fn part2_example() {
        let result = sum_ways_designs_are_producible("example.txt");
        assert_eq!(result, 16);
    }

    #[test]
    fn part2() {
        let result = sum_ways_designs_are_producible("input.txt");
        assert_eq!(result, 622121814629343);
    }
}
