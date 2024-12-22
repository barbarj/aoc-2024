#![allow(dead_code)]

use std::{collections::HashMap, fs};

use itertools::Itertools;

fn load_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string("input/2024/21/".to_owned() + filename).unwrap();
    contents.lines().map(|x| x.to_string()).collect()
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Vector2 {
    x: i8,
    y: i8,
}
impl Vector2 {
    fn new(x: i8, y: i8) -> Self {
        Vector2 { x, y }
    }
}

struct NumPadGenerator {
    position: Vector2,
}
impl NumPadGenerator {
    fn new() -> Self {
        NumPadGenerator {
            position: Vector2::new(2, 3),
        }
    }

    fn location_lookup(c: char) -> Vector2 {
        match c {
            '0' => Vector2::new(1, 3),
            'A' => Vector2::new(2, 3),
            '1' => Vector2::new(0, 2),
            '2' => Vector2::new(1, 2),
            '3' => Vector2::new(2, 2),
            '4' => Vector2::new(0, 1),
            '5' => Vector2::new(1, 1),
            '6' => Vector2::new(2, 1),
            '7' => Vector2::new(0, 0),
            '8' => Vector2::new(1, 0),
            '9' => Vector2::new(2, 0),
            _ => unreachable!(),
        }
    }

    fn next(&mut self, c: char) -> String {
        let next_pos = Self::location_lookup(c);
        let mut dx = next_pos.x - self.position.x;
        let mut dy = next_pos.y - self.position.y;
        let mut output = String::new();
        // order of direction preference: right, up, down, left
        while dx > 0 {
            output.push('>');
            dx -= 1;
        }
        while dy < 0 {
            output.push('^');
            dy += 1;
        }
        while dy > 0 {
            output.push('v');
            dy -= 1;
        }
        while dx < 0 {
            output.push('<');
            dx += 1;
        }
        output.push('A');

        self.position = next_pos;
        output
    }

    fn produce(&mut self, input: &str) -> String {
        let mut output = String::new();
        for c in input.chars() {
            output.push_str(&self.next(c));
        }
        output
    }
}

fn encounters_disallowed(path: &str, mut pos: Vector2, disallowed: &Vector2) -> bool {
    for c in path.chars() {
        match c {
            '>' => pos.x += 1,
            'v' => pos.y += 1,
            '<' => pos.x -= 1,
            '^' => pos.y -= 1,
            _ => unreachable!(),
        }
        if pos == *disallowed {
            return true;
        }
    }
    false
}

fn generate_paths(pos1: &Vector2, pos2: &Vector2, disallowed: &Vector2) -> Vec<String> {
    let mut dx = pos2.x - pos1.x;
    let mut dy = pos2.y - pos1.y;

    let mut path_base = String::new();
    while dx > 0 {
        path_base.push('>');
        dx -= 1;
    }
    while dy < 0 {
        path_base.push('^');
        dy += 1;
    }
    while dy > 0 {
        path_base.push('v');
        dy -= 1;
    }
    while dx < 0 {
        path_base.push('<');
        dx += 1;
    }
    path_base
        .chars()
        .permutations(path_base.len())
        .map(|chars| chars.into_iter().collect::<String>())
        .filter(|s: &String| !encounters_disallowed(s, pos1.clone(), disallowed))
        .map(|s| s + "A")
        .collect()
}

fn build_numpad_paths() -> HashMap<(char, char), Vec<String>> {
    let mut paths = HashMap::new();

    for c1 in "0123456789A".chars() {
        for c2 in "0123456789A".chars() {
            paths.insert(
                (c1, c2),
                generate_paths(
                    &NumPadGenerator::location_lookup(c1),
                    &NumPadGenerator::location_lookup(c2),
                    &Vector2::new(0, 3),
                ),
            );
        }
    }
    paths
}

fn build_keypad_paths() -> HashMap<(char, char), Vec<String>> {
    let mut paths = HashMap::new();

    for c1 in "<>^vA".chars() {
        for c2 in "<>^vA".chars() {
            paths.insert(
                (c1, c2),
                generate_paths(
                    &KeyPadGenerator::location_lookup(c1),
                    &KeyPadGenerator::location_lookup(c2),
                    &Vector2::new(0, 0),
                ),
            );
        }
    }
    paths
}

fn generate(
    memo: &mut HashMap<(char, char, usize), usize>,
    path: &str,
    paths: &HashMap<(char, char), Vec<String>>,
    level: usize,
) -> usize {
    if level == 0 {
        return path.len();
    }
    let mut current = 'A';
    let mut generated = 0;
    for c in path.chars() {
        let shortest = match memo.get(&(current, c, level)) {
            Some(s) => *s,
            None => {
                let paths0 = paths.get(&(current, c)).unwrap();
                let shortest = paths0
                    .iter()
                    .map(|p| generate(memo, p, paths, level - 1))
                    .min()
                    .unwrap();
                if level <= 20 {
                    memo.insert((current, c, level), shortest);
                }
                //println!("{level}: {}, {}", memo.len(), shortest.len());
                shortest
            }
        };
        generated += &shortest;
        current = c;
    }
    generated
}

fn shortest_generated_sequence_len(
    input: &str,
    levels: usize,
    memo: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    let numpad_paths = build_numpad_paths();
    let keypad_paths = build_keypad_paths();

    let mut current0 = 'A';
    let mut result = 0;
    for c in input.chars() {
        let paths0 = numpad_paths.get(&(current0, c)).unwrap();
        let shortest = paths0
            .iter()
            .map(|p| generate(memo, p, &keypad_paths, levels))
            .min()
            .unwrap();
        result += shortest;
        current0 = c;
    }
    result
}

struct KeyPadGenerator {
    pos: Vector2,
}
impl KeyPadGenerator {
    fn new() -> Self {
        KeyPadGenerator {
            pos: Vector2::new(2, 0),
        }
    }

    fn reset(&mut self) {
        self.pos = Vector2::new(2, 0);
    }

    fn location_lookup(c: char) -> Vector2 {
        match c {
            'A' => Vector2::new(2, 0),
            '^' => Vector2::new(1, 0),
            '<' => Vector2::new(0, 1),
            'v' => Vector2::new(1, 1),
            '>' => Vector2::new(2, 1),
            _ => unreachable!(),
        }
    }

    fn next(&mut self, c: char) -> String {
        let next_pos = Self::location_lookup(c);
        let mut dx = next_pos.x - self.pos.x;
        let mut dy = next_pos.y - self.pos.y;
        let mut output = String::new();
        // order of direction preference: right, up, down, left
        while dx > 0 {
            output.push('>');
            dx -= 1;
        }
        while dy < 0 {
            output.push('^');
            dy += 1;
        }
        while dy > 0 {
            output.push('v');
            dy -= 1;
        }
        while dx < 0 {
            output.push('<');
            dx += 1;
        }
        output.push('A');

        self.pos = next_pos;
        output
    }

    fn produce(&mut self, input: &str) -> String {
        let mut output = String::new();
        for c in input.chars() {
            output.push_str(&self.next(c));
        }
        output
    }
}

fn shortest_sequence(input: &str) -> String {
    let mut numpad = NumPadGenerator::new();
    let level1 = numpad.produce(input);
    let mut keypad1 = KeyPadGenerator::new();
    let level2 = keypad1.produce(&level1);
    let mut keypad2 = KeyPadGenerator::new();
    keypad2.produce(&level2)
}

fn num_part(s: &str) -> usize {
    s[..s.len() - 1].parse::<usize>().unwrap()
}

pub fn situation(filename: &str, levels: usize) -> usize {
    let inputs = load_input(filename);
    let mut memo = HashMap::new();
    inputs
        .iter()
        .map(|input| shortest_generated_sequence_len(input, levels, &mut memo) * num_part(input))
        .inspect(|size| println!("{size}"))
        .sum()
}

#[derive(Eq, PartialEq, Hash)]
struct Key {
    pos: Vector2,
    desired_pos: Vector2,
    level: usize,
}
impl Key {
    fn new(pos: Vector2, desired_pos: Vector2, level: usize) -> Self {
        Key {
            pos,
            desired_pos,
            level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{load_input, num_part, shortest_generated_sequence_len, situation};
    use std::collections::HashMap;

    #[test]
    fn first_example() {
        let mut memo = HashMap::new();
        let result = shortest_generated_sequence_len("029A", 2, &mut memo);
        assert_eq!(
            result,
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn last_example() {
        let mut memo = HashMap::new();
        let result = shortest_generated_sequence_len("379A", 2, &mut memo);
        assert_eq!(
            result,
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len(),
        );
    }

    #[test]
    fn num_parts() {
        let inputs = load_input("example.txt");
        let result: Vec<_> = inputs.iter().map(|s| num_part(s)).collect();
        assert_eq!(result, [29, 980, 179, 456, 379]);
    }

    #[test]
    fn seq_lens() {
        let inputs = load_input("example.txt");
        let mut memo = HashMap::new();
        let result: Vec<_> = inputs
            .iter()
            .map(|s| shortest_generated_sequence_len(s, 2, &mut memo))
            .collect();
        assert_eq!(result, [68, 60, 68, 64, 64]);
    }

    #[test]
    fn part1_example() {
        let result = situation("example.txt", 2);
        assert_eq!(result, 126384);
    }

    #[test]
    fn part1() {
        let result = situation("input.txt", 2);
        assert_eq!(result, 171596);
    }

    #[test]
    fn part2() {
        let result = situation("input.txt", 25);
        assert_eq!(result, 209268004868246);
    }
}
