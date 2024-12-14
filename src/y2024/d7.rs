#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input(filename: &str) -> Vec<(u64, Vec<u64>)> {
    let file = File::open("input/2024/7/".to_owned() + filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split(':');
            let total = parts.next().unwrap().parse::<u64>().unwrap();
            let values = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            (total, values)
        })
        .collect()
}

fn can_achieve_total(values: &[u64], goal: u64, allow_concat: bool, so_far: u64) -> bool {
    if values.is_empty() {
        return so_far == goal;
    }
    if so_far > goal {
        return false;
    }
    // special case that will only happen at beginning
    if so_far == 0 {
        return can_achieve_total(&values[1..], goal, allow_concat, values[0]);
    }
    if can_achieve_total(&values[1..], goal, allow_concat, so_far * values[0])
        || can_achieve_total(&values[1..], goal, allow_concat, so_far + values[0])
    {
        true
    } else if allow_concat {
        let concatenated = format!("{so_far}{}", values[0]).parse::<u64>().unwrap();
        can_achieve_total(&values[1..], goal, allow_concat, concatenated)
    } else {
        false
    }
}

fn sum_functions_that_work(filename: &str, allow_concat: bool) -> u64 {
    let functions = get_input(filename);
    functions
        .iter()
        .filter(|(total, values)| can_achieve_total(values, *total, allow_concat, 0))
        .map(|(total, _)| total)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::sum_functions_that_work;

    #[test]
    fn part1_example() {
        let result = sum_functions_that_work("example.txt", false);
        assert_eq!(result, 3749);
    }

    #[test]
    fn part1() {
        let result = sum_functions_that_work("input.txt", false);
        assert_eq!(result, 4364915411363);
    }

    #[test]
    fn part2_example() {
        let result = sum_functions_that_work("example.txt", true);
        assert_eq!(result, 11387);
    }

    #[test]
    fn part2() {
        let result = sum_functions_that_work("input.txt", true);
        assert_eq!(result, 38322057216320);
    }
}
