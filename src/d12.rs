#![allow(dead_code)]

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

struct Map<T: Copy> {
    map: Vec<T>,
    width: usize,
    height: usize,
}
impl<T: Copy> Map<T> {
    fn get(&self, i: usize, j: usize) -> T {
        let idx = (i * self.width) + j;
        self.map[idx]
    }

    fn set(&mut self, i: usize, j: usize, value: T) {
        let idx = (i * self.width) + j;
        self.map[idx] = value;
    }
}

fn load_input(filename: &str) -> Map<u8> {
    let file = File::open("input/12/".to_owned() + filename).unwrap();
    let mut map = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        height += 1;
        if width == 0 {
            width = line.trim().len();
        }
        map.extend(line.trim().bytes());
    }
    Map { map, width, height }
}

fn find_area_and_perimiter(map: &Map<u8>, seen: &mut Map<bool>, i: usize, j: usize) -> (u32, u32) {
    let kind = map.get(i, j);
    let mut queue = VecDeque::new();
    queue.push_back((i, j));
    let mut area = 0;
    let mut perimiter = 0;
    while let Some((i, j)) = queue.pop_front() {
        if seen.get(i, j) {
            continue;
        }
        seen.set(i, j, true);
        area += 1;
        // up
        if i > 0 && map.get(i - 1, j) == kind {
            queue.push_back((i - 1, j));
        } else {
            perimiter += 1;
        }
        // right
        if j < map.width - 1 && map.get(i, j + 1) == kind {
            queue.push_back((i, j + 1));
        } else {
            perimiter += 1;
        }
        // down
        if i < map.height - 1 && map.get(i + 1, j) == kind {
            queue.push_back((i + 1, j));
        } else {
            perimiter += 1;
        }
        // left
        if j > 0 && map.get(i, j - 1) == kind {
            queue.push_back((i, j - 1));
        } else {
            perimiter += 1;
        }
    }
    (area, perimiter)
}

fn is_concave_corner_up_right(map: &Map<u8>, i: usize, j: usize) -> bool {
    let kind = map.get(i, j);
    i > 0
        && j < map.width - 1
        && map.get(i - 1, j + 1) != kind
        && map.get(i - 1, j) == kind
        && map.get(i, j + 1) == kind
}

fn is_concave_corner_up_left(map: &Map<u8>, i: usize, j: usize) -> bool {
    let kind = map.get(i, j);
    i > 0
        && j > 0
        && map.get(i - 1, j - 1) != kind
        && map.get(i - 1, j) == kind
        && map.get(i, j - 1) == kind
}

fn is_concave_corner_down_left(map: &Map<u8>, i: usize, j: usize) -> bool {
    let kind = map.get(i, j);
    i < map.height - 1
        && j > 0
        && map.get(i + 1, j - 1) != kind
        && map.get(i + 1, j) == kind
        && map.get(i, j - 1) == kind
}

fn is_concave_corner_down_right(map: &Map<u8>, i: usize, j: usize) -> bool {
    let kind = map.get(i, j);
    i < map.height - 1
        && j < map.width - 1
        && map.get(i + 1, j + 1) != kind
        && map.get(i + 1, j) == kind
        && map.get(i, j + 1) == kind
}

fn find_area_and_sides(map: &Map<u8>, seen: &mut Map<bool>, i: usize, j: usize) -> (u32, u32) {
    let kind = map.get(i, j);
    let mut queue = VecDeque::new();
    queue.push_back((i, j));
    let mut area = 0;
    let mut corners = 0;

    while let Some((i, j)) = queue.pop_front() {
        if seen.get(i, j) {
            continue;
        }
        seen.set(i, j, true);
        area += 1;
        let mut edges = 0u8;
        // up
        if i > 0 && map.get(i - 1, j) == kind {
            queue.push_back((i - 1, j));
        } else {
            edges |= 0b0001;
        }
        // right
        if j < map.width - 1 && map.get(i, j + 1) == kind {
            queue.push_back((i, j + 1));
        } else {
            edges |= 0b0010;
        }
        // down
        if i < map.height - 1 && map.get(i + 1, j) == kind {
            queue.push_back((i + 1, j));
        } else {
            edges |= 0b0100;
        }
        // left
        if j > 0 && map.get(i, j - 1) == kind {
            queue.push_back((i, j - 1));
        } else {
            edges |= 0b1000;
        }

        // convex corners
        let mut new_corners = match edges {
            0b0011 | 0b0110 | 0b1100 | 0b1001 => 1,
            0b0111 | 0b1110 | 0b1101 | 0b1011 => 2,
            0b1111 => 4,
            _ => 0,
        };
        // concave corners
        new_corners += [
            is_concave_corner_up_left(map, i, j),
            is_concave_corner_up_right(map, i, j),
            is_concave_corner_down_left(map, i, j),
            is_concave_corner_down_right(map, i, j),
        ]
        .iter()
        .filter(|x| **x)
        .count() as u32;

        corners += new_corners;
    }

    (area, corners)
}

fn fence_price_using_perimiter(filename: &str) -> u32 {
    let map = load_input(filename);
    let mut seen = Map {
        map: vec![false; map.map.len()],
        width: map.width,
        height: map.height,
    };
    let mut price_sum = 0;
    for i in 0..map.height {
        for j in 0..map.width {
            if seen.get(i, j) {
                continue;
            }
            let (area, perimiter) = find_area_and_perimiter(&map, &mut seen, i, j);
            price_sum += area * perimiter;
        }
    }
    price_sum
}

fn fence_price_using_sides(filename: &str) -> u32 {
    let map = load_input(filename);
    let mut seen = Map {
        map: vec![false; map.map.len()],
        width: map.width,
        height: map.height,
    };
    let mut price_sum = 0;
    for i in 0..map.height {
        for j in 0..map.width {
            if seen.get(i, j) {
                continue;
            }
            let (area, sides) = find_area_and_sides(&map, &mut seen, i, j);
            price_sum += area * sides;
        }
    }
    price_sum
}

#[cfg(test)]
mod tests {
    use crate::d12::{fence_price_using_perimiter, fence_price_using_sides};

    #[test]
    fn part1_example() {
        let result = fence_price_using_perimiter("example.txt");
        assert_eq!(result, 1930);
    }

    #[test]
    fn part1() {
        let result = fence_price_using_perimiter("input.txt");
        assert_eq!(result, 1494342);
    }

    #[test]
    fn part2_example() {
        let result = fence_price_using_sides("example.txt");
        assert_eq!(result, 1206);
    }

    #[test]
    fn part2() {
        let result = fence_price_using_sides("input.txt");
        assert_eq!(result, 893676);
    }
}
