#![allow(dead_code)]
use std::fs;

use regex::Regex;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    fn from_str(x: &str, y: &str) -> Self {
        Self::new(x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    }

    fn translate_by_pos(&mut self, pos: &Self) {
        self.x += pos.x;
        self.y += pos.y;
    }

    fn translate_by_neg(&mut self, pos: &Self) {
        self.x -= pos.x;
        self.y -= pos.y;
    }

    fn maximum_interval(&self, int: &Self) -> usize {
        (self.x / int.x).min(self.y / int.y)
    }

    fn before_pos(&self, pos: &Self) -> bool {
        self.x < pos.x && self.y < pos.y
    }
}

// returns (delta a, delta b, goal position)
fn load_input(filename: &str) -> Vec<(Position, Position, Position)> {
    let contents = fs::read_to_string("input/13/".to_owned() + filename).unwrap();
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

const PRIMES: [usize; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(mut a: usize, mut b: usize) -> usize {
    //println!("lcm({a}, {b}) = ");
    for p in &PRIMES {
        if b < *p {
            break;
        }
        while b >= *p && b % p == 0 {
            b /= p;
            a *= p;
        }
    }
    a *= b;
    //println!("{a}");
    a
}

fn valid(a_diff: &Position, b_diff: &Position, prize: &Position, a: usize, b: usize) -> bool {
    (a * a_diff.x) + (b * b_diff.x) == prize.x && (a * a_diff.y) + (b * b_diff.y) == prize.y
}

fn cheapest_path_to_prize(a_diff: &Position, b_diff: &Position, prize: &Position) -> Option<usize> {
    println!("{a_diff:?}, {b_diff:?}, {prize:?}");

    let p = prize.x + prize.y;
    let combined_a_diff = a_diff.x + a_diff.y;
    let combined_b_diff = b_diff.x + b_diff.y;
    //println!("{p} = {combined_a_diff}a + {combined_b_diff}b");
    let comb_gcd = gcd(combined_a_diff, combined_b_diff);
    //println!("gcd: {comb_gcd}");
    if p % comb_gcd != 0 {
        return None;
    }
    let a_step = combined_b_diff / comb_gcd;

    let a_from_b = |bb| (p - (bb * combined_b_diff)) / combined_a_diff;
    let b_from_a = |aa| (p - (aa * combined_a_diff)) / combined_b_diff;

    // maximize a
    let mut b = 0;
    while b * combined_b_diff <= p && (p - (b * combined_b_diff)) % combined_a_diff != 0 {
        b += 1;
    }
    if b * combined_b_diff > p {
        return None;
    }
    let mut min: Option<usize> = None;
    let mut a = a_from_b(b);
    while a >= a_step {
        b = b_from_a(a);
        if valid(a_diff, b_diff, prize, a, b) {
            println!("valid: {a}, {b} = {}", (3 * a) + b);
            return Some((3 * a) + b);
            //min = min.map(|m| m.min((3 * a) + b)).or(Some((3 * a) + b));
        }
        a -= a_step;
    }
    b = b_from_a(a);
    if valid(a_diff, b_diff, prize, a, b) {
        println!("valid: {a}, {b} = {}", (3 * a) + b);
        return Some((3 * a) + b);
        //min = min.map(|m| m.min((3 * a) + b)).or(Some((3 * a) + b));
    }
    min
}

fn sum_cheapest_paths(filename: &str) -> usize {
    let input = load_input(filename);
    input
        .iter()
        .filter_map(|(a, b, prize)| cheapest_path_to_prize(a, b, prize))
        .sum()
}

pub fn sum_cheapest_paths_part2(filename: &str) -> usize {
    let input = load_input(filename);
    println!("{} inputs", input.len());
    input
        .into_iter()
        .map(|(a, b, mut prize)| {
            prize.translate_by_pos(&Position::new(10000000000000, 10000000000000));
            (a, b, prize)
        })
        .filter_map(|(a, b, prize)| cheapest_path_to_prize(&a, &b, &prize))
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
        assert_eq!(result, 480);
    }

    #[test]
    fn part2_problem() {
        panic!("Don't run in tests, takes way too long");
        let result = sum_cheapest_paths_part2("input.txt");
        assert_eq!(result, 82261957837868);
    }
}
