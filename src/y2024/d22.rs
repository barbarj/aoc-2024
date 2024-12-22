#![allow(dead_code)]

use std::{
    collections::HashSet,
    fs,
    sync::{Arc, RwLock},
    thread,
};

use itertools::Itertools;

fn get_inputs(filename: &str) -> Vec<i64> {
    let contents = fs::read_to_string("input/2024/22/".to_owned() + filename).unwrap();
    contents
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect()
}

struct SecretNumberSeq {
    num: i64,
}
impl SecretNumberSeq {
    fn new(num: i64) -> Self {
        SecretNumberSeq { num }
    }

    fn sequences(self) -> PriceChangeSeqs {
        PriceChangeSeqs::new(self)
    }
}
impl Iterator for SecretNumberSeq {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        let output = self.num;
        self.num ^= self.num * 64;
        self.num %= 16777216;
        self.num ^= self.num / 32;
        self.num %= 16777216;
        self.num ^= self.num * 2048;
        self.num %= 16777216;
        Some(output)
    }
}

struct PriceChangeSeqs {
    nums: SecretNumberSeq,
    seq: Seq4,
    price: i64,
}
impl PriceChangeSeqs {
    fn new(mut nums: SecretNumberSeq) -> Self {
        let a = nums.next().unwrap() % 10;
        let b = nums.next().unwrap() % 10;
        let c = nums.next().unwrap() % 10;
        let d = nums.next().unwrap() % 10;
        let e = nums.next().unwrap() % 10;
        let seq = Seq4::new(b - a, c - b, d - c, e - d);

        PriceChangeSeqs {
            nums,
            seq,
            price: e,
        }
    }
}
impl Iterator for PriceChangeSeqs {
    type Item = (Seq4, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let out = (self.seq.clone(), self.price);
        let new_price = self.nums.next().unwrap() % 10;
        let new_diff = new_price - self.price;
        self.seq.slide(new_diff);
        self.price = new_price;
        Some(out)
    }
}

fn sum_secret_numbers_after_steps(filename: &str, steps: usize) -> i64 {
    let nums = get_inputs(filename);
    nums.into_iter()
        .map(|n| SecretNumberSeq::new(n).nth(steps).unwrap())
        .sum()
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Seq4 {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}
impl Seq4 {
    fn new(a: i64, b: i64, c: i64, d: i64) -> Self {
        Seq4 { a, b, c, d }
    }

    fn slide(&mut self, next: i64) {
        self.a = self.b;
        self.b = self.c;
        self.c = self.d;
        self.d = next;
    }
}

fn score_seq(starting_nums: &[i64], seq: &Seq4) -> i64 {
    starting_nums
        .iter()
        .map(|n| {
            SecretNumberSeq::new(*n)
                .sequences()
                .take(1996)
                .find(|(s, _)| s == seq)
                .map(|(_, p)| p)
                .unwrap_or(0)
        })
        .sum()
}

fn seq_set(nums: &[i64]) -> HashSet<Seq4> {
    nums.iter()
        .flat_map(|n| {
            SecretNumberSeq::new(*n)
                .sequences()
                .take(1995)
                .map(|(s, _)| s)
        })
        .collect()
}

pub fn sell_for_bananas(filename: &str, thread_count: usize) -> i64 {
    let nums = get_inputs(filename);
    let sequences = seq_set(&nums);
    let seq_len = sequences.len();
    let starting_nums = Arc::new(RwLock::new(nums));
    let seq_chunks = sequences.into_iter().chunks(seq_len / (thread_count - 1));
    let mut chunk_iter = seq_chunks.into_iter();

    let mut handles = Vec::new();
    for _ in 0..thread_count {
        let seqgen: Vec<_> = chunk_iter.next().unwrap().collect();
        let nums_lock = starting_nums.clone();
        let h = thread::spawn(move || {
            let nums = nums_lock.read().unwrap();
            let mut max = 0;
            for seq in seqgen {
                let score = score_seq(&nums, &seq);
                max = max.max(score);
            }
            max
        });
        handles.push(h);
    }
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    results.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::sum_secret_numbers_after_steps;
    use super::{sell_for_bananas, SecretNumberSeq, Seq4};

    #[test]
    fn part1_example() {
        let result = sum_secret_numbers_after_steps("example.txt", 2000);
        assert_eq!(result, 37327623);
    }

    #[test]
    fn part1() {
        let result = sum_secret_numbers_after_steps("input.txt", 2000);
        assert_eq!(result, 17965282217);
    }

    #[test]
    fn price_change_sequence() {
        let seq: Vec<_> = SecretNumberSeq::new(123).sequences().take(6).collect();
        let expected = vec![
            (Seq4::new(-3, 6, -1, -1), 4),
            (Seq4::new(6, -1, -1, 0), 4),
            (Seq4::new(-1, -1, 0, 2), 6),
            (Seq4::new(-1, 0, 2, -2), 4),
            (Seq4::new(0, 2, -2, 0), 4),
            (Seq4::new(2, -2, 0, -2), 2),
        ];
        assert_eq!(seq, expected);
    }

    #[test]
    fn part2_example() {
        let result = sell_for_bananas("example2.txt", 3);
        assert_eq!(result, 23);
    }

    //#[test]
    fn part2() {
        // not currently tested because it takes 90 seconds for even the release build to run
        let result = sell_for_bananas("example2.txt", 12);
        assert_eq!(result, 2152);
    }
}
