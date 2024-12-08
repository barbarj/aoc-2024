#![allow(dead_code)]

// plan
// - parse input into hashmap of antenna kind and locations.
// - for every antenna kind:
//      - for every antenna after this one:
//          - determine antinode positions based on the position of these two.
//          - if they are on the map, add them to the positions set

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

type Position = (i32, i32);

#[derive(Debug)]
struct Map {
    width: i32,
    height: i32,
    positions: HashMap<char, Vec<Position>>,
}

fn parse_input(filename: &str) -> Map {
    let file = File::open("input/8/".to_owned() + filename).unwrap();
    let mut positions = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (r, line) in BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        width = line.len().try_into().unwrap();
        height += 1;
        for (c, ch) in line.chars().enumerate() {
            if ch != '.' {
                positions
                    .entry(ch)
                    .and_modify(|v: &mut Vec<Position>| {
                        v.push((r.try_into().unwrap(), c.try_into().unwrap()));
                    })
                    .or_insert(vec![(r.try_into().unwrap(), c.try_into().unwrap())]);
            }
        }
    }

    Map {
        width,
        height,
        positions,
    }
}

fn contained_by(p: Position, width: i32, height: i32) -> bool {
    p.0 >= 0 && p.0 < height && p.1 >= 0 && p.1 < width
}

fn find_antinodes(positions: &[Position], width: i32, height: i32) -> Vec<Position> {
    let mut antinodes = Vec::new();
    for (i, p1) in positions.iter().enumerate() {
        for p2 in positions[i + 1..].iter() {
            let (rise, run) = (p2.0 - p1.0, p2.1 - p1.1);
            let np_before = (p1.0 - rise, p1.1 - run);
            if contained_by(np_before, width, height) {
                antinodes.push(np_before);
            }
            let np_after = (p2.0 + rise, p2.1 + run);
            if contained_by(np_after, width, height) {
                antinodes.push(np_after);
            }
        }
    }
    antinodes
}

fn find_antinodes_part2(positions: &[Position], width: i32, height: i32) -> Vec<Position> {
    let mut antinodes = Vec::new();
    for (i, p1) in positions.iter().enumerate() {
        for p2 in positions[i + 1..].iter() {
            antinodes.push(*p1);
            let (rise, run) = (p2.0 - p1.0, p2.1 - p1.1);
            let mut np = (p1.0 - rise, p1.1 - run);
            while contained_by(np, width, height) {
                antinodes.push(np);
                np = (np.0 - rise, np.1 - run);
            }
            np = (p1.0 + rise, p1.1 + run);
            while contained_by(np, width, height) {
                antinodes.push(np);
                np = (np.0 + rise, np.1 + run);
            }
        }
    }
    antinodes
}

fn count_distinct_antinodes(filename: &str) -> usize {
    let map = parse_input(filename);
    let distinct_antinodes: HashSet<Position> = map
        .positions
        .values()
        .flat_map(|positions| find_antinodes(positions, map.width, map.height))
        .collect();
    distinct_antinodes.len()
}

fn count_distinct_antinodes_part2(filename: &str) -> usize {
    let map = parse_input(filename);
    let distinct_antinodes: HashSet<Position> = map
        .positions
        .values()
        .flat_map(|positions| find_antinodes_part2(positions, map.width, map.height))
        .collect();
    distinct_antinodes.len()
}

#[cfg(test)]
mod tests {
    use crate::d8::count_distinct_antinodes_part2;

    use super::count_distinct_antinodes;

    #[test]
    fn part1_example() {
        let result = count_distinct_antinodes("example.txt");
        assert_eq!(result, 14);
    }

    #[test]
    fn part1() {
        let result = count_distinct_antinodes("input.txt");
        assert_eq!(result, 327);
    }

    #[test]
    fn part2_example() {
        let result = count_distinct_antinodes_part2("example.txt");
        assert_eq!(result, 34);
    }

    #[test]
    fn part2() {
        let result = count_distinct_antinodes_part2("input.txt");
        assert_eq!(result, 1233);
    }
}
