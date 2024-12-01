use std::collections::HashMap;

use crate::common::lines_from_file;

#[allow(dead_code)]
fn list_differences(filename: &str) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines_from_file(filename) {
        let mut split = line.split("   ");
        let left_num = split.next().unwrap();
        let right_num = split.next().unwrap();
        left.push(left_num.parse().unwrap());
        right.push(right_num.parse().unwrap());
    }

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[allow(dead_code)]
fn similarity_score(filename: &str) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    for line in lines_from_file(filename) {
        let mut split = line.split("   ");
        let left_num = split.next().unwrap().parse().unwrap();
        let right_num = split.next().unwrap().parse().unwrap();
        left.push(left_num);
        right.entry(right_num).and_modify(|e| *e += 1).or_insert(1);
    }

    left.iter().map(|x| x * right.get(x).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::{list_differences, similarity_score};

    #[test]
    fn part1_example() {
        let result = list_differences("1/example.txt");
        assert_eq!(result, 11);
    }

    #[test]
    fn part1() {
        let result = list_differences("1/input.txt");
        assert_eq!(result, 1590491);
    }

    #[test]
    fn part2_example() {
        let result = similarity_score("1/example.txt");
        assert_eq!(result, 31);
    }

    #[test]
    fn part2() {
        let result = similarity_score("1/input.txt");
        assert_eq!(result, 22588371);
    }
}
