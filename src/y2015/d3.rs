#![allow(dead_code)]

use std::{collections::HashSet, fs};

fn load_input(filename: &str) -> String {
    fs::read_to_string("input/2015/3/".to_owned() + filename).unwrap()
}

fn get_visited(directions: impl Iterator<Item = char>) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();
    let mut position = (0, 0);
    visited.insert(position);
    for c in directions {
        match c {
            '>' => position.0 += 1,
            'v' => position.1 += 1,
            '<' => position.0 -= 1,
            '^' => position.1 -= 1,
            _ => unreachable!(),
        }
        visited.insert(position);
    }
    visited
}

fn one_person(filename: &str) -> usize {
    let input = load_input(filename);
    let visited = get_visited(input.trim().chars());
    visited.len()
}

fn two_people(filename: &str) -> usize {
    let input = load_input(filename);
    let dir1 = input
        .trim()
        .chars()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .map(|(_, c)| c);
    let dir2 = input
        .trim()
        .chars()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 1)
        .map(|(_, c)| c);
    let set1 = get_visited(dir1);
    let set2 = get_visited(dir2);
    set1.union(&set2).count()
}

#[cfg(test)]
mod tests {
    use super::{one_person, two_people};

    #[test]
    fn part1() {
        let result = one_person("input.txt");
        assert_eq!(result, 2592);
    }

    #[test]
    fn part2() {
        let result = two_people("input.txt");
        assert_eq!(result, 2360);
    }
}
