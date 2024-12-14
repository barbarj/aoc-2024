#![allow(dead_code)]
use std::fs;

use regex::Regex;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Position {
    x: i64,
    y: i64,
}
impl Position {
    fn new(x: i64, y: i64) -> Self {
        Position { x, y }
    }

    fn from_str(x: &str, y: &str) -> Self {
        Self::new(x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
    }

    fn translate_by_pos(&mut self, pos: &Self) {
        self.x += pos.x;
        self.y += pos.y;
    }
}

// returns (delta a, delta b, goal position)
fn load_input(filename: &str) -> Vec<(Position, Position, Position)> {
    let contents = fs::read_to_string("input/2024/13/".to_owned() + filename).unwrap();
    let mut lines = contents.lines().peekable();
    let mut inputs = Vec::new();

    let pattern_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let pattern_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let pattern_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    while lines.peek().is_some() {
        let a_captures = pattern_a.captures(lines.next().unwrap()).unwrap();
        let a = Position::from_str(&a_captures[1], &a_captures[2]);
        let b_captures = pattern_b.captures(lines.next().unwrap()).unwrap();
        let b = Position::from_str(&b_captures[1], &b_captures[2]);
        let prize_captures = pattern_prize.captures(lines.next().unwrap()).unwrap();
        let prize = Position::from_str(&prize_captures[1], &prize_captures[2]);
        inputs.push((a, b, prize));
        lines.next(); // skip empty line
    }
    inputs
}

fn valid(a_diff: &Position, b_diff: &Position, prize: &Position, a: i64, b: i64) -> bool {
    (a * a_diff.x) + (b * b_diff.x) == prize.x && (a * a_diff.y) + (b * b_diff.y) == prize.y
}

struct LinearEquation {
    a: i64,
    b: i64,
    c: i64,
}
impl LinearEquation {
    fn new(a: i64, b: i64, c: i64) -> Self {
        LinearEquation { a, b, c }
    }

    fn multiply_by(&self, x: i64) -> Self {
        LinearEquation {
            a: self.a * x,
            b: self.b * x,
            c: self.c * x,
        }
    }

    fn subtract_equation(&self, other: &LinearEquation) -> Self {
        LinearEquation {
            a: self.a - other.a,
            b: self.b - other.b,
            c: self.c - other.c,
        }
    }
}

fn path_to_prize_fast_way(a_diff: &Position, b_diff: &Position, prize: &Position) -> Option<i64> {
    let x_eq = LinearEquation::new(a_diff.x, b_diff.x, prize.x);
    let y_eq = LinearEquation::new(a_diff.y, b_diff.y, prize.y);

    let x_by_ay = x_eq.multiply_by(y_eq.a);
    let y_by_ax = y_eq.multiply_by(x_eq.a);

    let b_only_eq = x_by_ay.subtract_equation(&y_by_ax);
    let b = b_only_eq.c / b_only_eq.b;

    let a = (x_eq.c - (x_eq.b * b)) / x_eq.a;
    if valid(a_diff, b_diff, prize, a, b) {
        Some((a * 3) + b)
    } else {
        None
    }
}

fn sum_cheapest_paths(filename: &str) -> i64 {
    let input = load_input(filename);
    input
        .iter()
        .filter_map(|(a, b, prize)| path_to_prize_fast_way(a, b, prize))
        .sum()
}

pub fn sum_cheapest_paths_part2(filename: &str) -> i64 {
    let input = load_input(filename);
    input
        .into_iter()
        .map(|(a, b, mut prize)| {
            prize.translate_by_pos(&Position::new(10000000000000, 10000000000000));
            (a, b, prize)
        })
        .filter_map(|(a, b, prize)| path_to_prize_fast_way(&a, &b, &prize))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{sum_cheapest_paths, sum_cheapest_paths_part2};

    #[test]
    fn part1_example() {
        let result = sum_cheapest_paths("example.txt");
        assert_eq!(result, 480);
    }

    #[test]
    fn part1() {
        let result = sum_cheapest_paths("input.txt");
        assert_eq!(result, 33921);
    }

    #[test]
    fn part2_example() {
        let result = sum_cheapest_paths_part2("example.txt");
        assert_eq!(result, 875318608908);
    }

    #[test]
    fn part2_problem() {
        let result = sum_cheapest_paths_part2("input.txt");
        assert_eq!(result, 82261957837868);
    }
}
