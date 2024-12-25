#![allow(dead_code)]

// can probably speed this representation up by using a flat array
/*
struct KeyTrie {
    trie: [[[[[bool; 6]; 6]; 6]; 6]; 6],
}
impl KeyTrie {
    fn new() -> Self {
        let level5 = [false; 6];
        let level4 = [level5; 6];
        let level3 = [level4; 6];
        let level2 = [level3; 6];
        let level1 = [level2; 6];

        KeyTrie { trie: level1 }
    }

    fn insert_key(&mut self, key: u16) {
        let l1 = (key / 10000) as usize;
        let l2 = ((key / 1000) % 10) as usize;
        let l3 = ((key / 100) % 10) as usize;
        let l4 = ((key / 10) % 10) as usize;
        let l5 = (key % 10) as usize;

        assert!(l1 < 6);
        assert!(l2 < 6);
        assert!(l3 < 6);
        assert!(l4 < 6);
        assert!(l5 < 6);

        self.trie[l1][l2][l3][l4][l5] = true;
    }

    fn contains_key(&self, key: u16) -> bool {
        let l1 = (key / 10000) as usize;
        let l2 = ((key / 1000) % 10) as usize;
        let l3 = ((key / 100) % 10) as usize;
        let l4 = ((key / 10) % 10) as usize;
        let l5 = (key % 10) as usize;

        assert!(l1 < 6);
        assert!(l2 < 6);
        assert!(l3 < 6);
        assert!(l4 < 6);
        assert!(l5 < 6);

        self.trie[l1][l2][l3][l4][l5]
    }
}
*/

// can be made smaller by getting rid of 6-9 space of each digit, and treating input as a base-6
// number to get position
//type KeyList = [bool; 60000];

use std::{collections::HashSet, fs};

fn parse_lock(s: &str) -> u16 {
    let mut num = 0;
    let lines = s.split_whitespace();
    let grid: Vec<Vec<char>> = lines.skip(1).map(|l| l.chars().collect()).collect();
    for x in 0..5 {
        num *= 10;
        let mut y = 0;
        while y < 6 && grid[y][x] == '#' {
            num += 1;
            y += 1;
        }
    }
    num
}

fn parse_key(s: &str) -> u16 {
    let mut num = 0;
    let lines = s.split_whitespace();
    let grid: Vec<Vec<char>> = lines.take(6).map(|l| l.chars().collect()).collect();
    for x in 0..5 {
        num *= 10;
        let mut y = 6;
        while y > 0 && grid[y - 1][x] == '#' {
            num += 1;
            y -= 1;
        }
    }
    num
}

fn parse_input(filename: &str) -> (HashSet<u16>, HashSet<u16>) {
    let contents = fs::read_to_string("input/2024/25/".to_owned() + filename).unwrap();

    let mut locks = HashSet::new();
    let mut keys = HashSet::new();

    for block in contents.split("\n\n") {
        if block.starts_with("#") {
            locks.insert(parse_lock(block));
        } else {
            keys.insert(parse_key(block));
        }
    }

    (locks, keys)
}

fn digits(num: u16) -> (u16, u16, u16, u16, u16) {
    let d1 = num / 10000;
    let d2 = (num / 1000) % 10;
    let d3 = (num / 100) % 10;
    let d4 = (num / 10) % 10;
    let d5 = num % 10;

    (d1, d2, d3, d4, d5)
}

fn digits_to_num(d1: u16, d2: u16, d3: u16, d4: u16, d5: u16) -> u16 {
    let mut num = d1;
    num *= 10;
    num += d2;
    num *= 10;
    num += d3;
    num *= 10;
    num += d4;
    num *= 10;
    num += d5;
    num
}

fn count_unique_fits(filename: &str) -> usize {
    let (locks, keys) = parse_input(filename);

    let mut unique_fits = 0;
    for lock in locks {
        let (d1, d2, d3, d4, d5) = digits(lock);
        for k1 in 0..=(5 - d1) {
            for k2 in 0..=(5 - d2) {
                for k3 in 0..=(5 - d3) {
                    for k4 in 0..=(5 - d4) {
                        for k5 in 0..=(5 - d5) {
                            let key = digits_to_num(k1, k2, k3, k4, k5);
                            if keys.contains(&key) {
                                unique_fits += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    unique_fits
}

#[cfg(test)]
mod tests {
    use crate::y2024::d25::count_unique_fits;

    #[test]
    fn part1_example() {
        let result = count_unique_fits("example.txt");
        assert_eq!(result, 3);
    }

    #[test]
    fn part1() {
        let result = count_unique_fits("input.txt");
        assert_eq!(result, 3365);
    }
}
