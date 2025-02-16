#![allow(dead_code)]

use std::{collections::HashMap, fs};

fn pairs(s: &str) -> impl Iterator<Item = (char, char)> + '_ {
    s.chars().take(s.len() - 1).zip(s.chars().skip(1))
}

fn triplets(s: &str) -> impl Iterator<Item = (char, char, char)> + '_ {
    s.chars()
        .take(s.len() - 2)
        .zip(s.chars().skip(1).take(s.len() - 1))
        .zip(s.chars().skip(2))
        .map(|((a, b), c)| (a, b, c))
}

fn contains_at_least_3_vowels(s: &str) -> bool {
    s.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
}

fn contains_repeated_letter(s: &str) -> bool {
    pairs(s).any(|(a, b)| a == b)
}

fn contains_forbidden_substr(s: &str) -> bool {
    pairs(s).any(|(a, b)| {
        (a, b) == ('a', 'b') || (a, b) == ('c', 'd') || (a, b) == ('p', 'q') || (a, b) == ('x', 'y')
    })
}

fn contains_nonoverlapping_repeated_pair(s: &str) -> bool {
    let mut indices: HashMap<(char, char), Vec<usize>> = HashMap::new();
    for (idx, (a, b)) in pairs(s).enumerate() {
        indices
            .entry((a, b))
            .and_modify(|list| list.push(idx))
            .or_insert(vec![idx]);
    }
    for idx_list in indices.values() {
        let mut diffs = idx_list
            .iter()
            .take(idx_list.len() - 1)
            .zip(idx_list.iter().skip(1))
            .map(|(i1, i2)| i2 - i1);
        if diffs.any(|x| x > 1) {
            return true;
        }
    }

    false
}

fn contains_triplet_sandwhich(s: &str) -> bool {
    triplets(s).any(|(a, _, c)| a == c)
}

fn is_nice_string_part1(s: &str) -> bool {
    contains_at_least_3_vowels(s) && contains_repeated_letter(s) && !contains_forbidden_substr(s)
}

fn is_nice_string_part2(s: &str) -> bool {
    contains_nonoverlapping_repeated_pair(s) && contains_triplet_sandwhich(s)
}

fn get_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string("input/2015/5/".to_owned() + filename).unwrap();
    contents.split_whitespace().map(|x| x.to_string()).collect()
}

fn count_nice_strings_part1(filename: &str) -> usize {
    let strings = get_input(filename);
    strings.iter().filter(|s| is_nice_string_part1(s)).count()
}

fn count_nice_strings_part2(filename: &str) -> usize {
    let strings = get_input(filename);
    strings.iter().filter(|s| is_nice_string_part2(s)).count()
}

#[cfg(test)]
mod tests {
    use super::{count_nice_strings_part1, count_nice_strings_part2};

    #[test]
    fn part1_example() {
        let result = count_nice_strings_part1("example.txt");
        assert_eq!(result, 2);
    }

    #[test]
    fn part1() {
        let result = count_nice_strings_part1("input.txt");
        assert_eq!(result, 258);
    }

    #[test]
    fn part2() {
        let result = count_nice_strings_part2("input.txt");
        assert_eq!(result, 53);
    }
}
