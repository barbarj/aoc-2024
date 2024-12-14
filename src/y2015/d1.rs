#![allow(dead_code)]
use std::fs;

fn load_input(filename: &str) -> String {
    fs::read_to_string("input/2015/1/".to_owned() + filename).unwrap()
}

fn find_floor(filename: &str) -> i32 {
    let input = load_input(filename);
    input
        .trim()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .sum()
}

fn find_first_basement(filename: &str) -> i32 {
    let input = load_input(filename);
    let mut floor = 0;
    for (idx, c) in input.trim().chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        };
        if floor == -1 {
            return idx as i32 + 1;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::{find_first_basement, find_floor};

    #[test]
    fn part1_example() {
        let result = find_floor("example.txt");
        assert_eq!(result, -3);
    }

    #[test]
    fn part1() {
        let result = find_floor("input.txt");
        assert_eq!(result, 232);
    }

    #[test]
    fn part2() {
        let result = find_first_basement("input.txt");
        assert_eq!(result, 1783);
    }
}
