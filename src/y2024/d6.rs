#![allow(dead_code)]
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    thread,
};

type Position = (usize, usize);

#[derive(Clone)]
struct Map {
    map: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}
impl Map {
    fn new(map: Vec<Vec<u8>>) -> Self {
        let height = map.len();
        let width = map[0].len();
        Map { map, width, height }
    }

    fn add_obstacle(&mut self, (row, col): Position) {
        self.map[row][col] = b'#';
    }

    fn remove_obstacle(&mut self, (row, col): Position) {
        self.map[row][col] = 0;
    }
}

fn get_input(filename: &str) -> (Map, Position) {
    let file = File::open("input/2024/6/".to_owned() + filename).unwrap();

    let mut map: Vec<Vec<u8>> = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .bytes()
                .map(|b| if b == b'.' { 0 } else { b })
                .collect()
        })
        .collect();
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == b'^' {
                map[r][c] = 0;
                return (Map::new(map), (r, c));
            }
        }
    }
    unreachable!();
}

fn count_distinct_positions(filename: &str) -> usize {
    let (mut map_data, starting_pos) = get_input(filename);
    let visited = positions_visited(&mut map_data, starting_pos).unwrap();
    let unique: HashSet<_> = visited.iter().collect();
    unique.len()
}

fn clear_visited(map: &mut Map, visited: impl Iterator<Item = Position>) {
    for (row, col) in visited {
        map.map[row][col] = 0;
    }
}

fn count_loopable_obstacle_insertions(filename: &str, num_threads: usize) -> usize {
    let (mut map, starting_pos) = get_input(filename);
    let visited = positions_visited(&mut map, starting_pos).unwrap();
    let visited: HashSet<_> = visited.into_iter().collect();
    let visited: Vec<Position> = visited.into_iter().collect();
    clear_visited(&mut map, visited.iter().copied());
    let mut children = Vec::new();

    let mut chunk_size = visited.len() / num_threads;
    if visited.len() % chunk_size != 0 {
        chunk_size += 1;
    }
    let mut visited_chunks = visited.chunks(chunk_size);
    for _ in 0..num_threads {
        let mut map = map.clone();
        let positions: Vec<Position> = visited_chunks.next().unwrap().to_vec();
        let child = thread::spawn(move || {
            let mut count = 0;
            for obs_pos in positions {
                if starting_pos == obs_pos {
                    continue;
                }
                map.add_obstacle(obs_pos);
                match positions_visited(&mut map, starting_pos) {
                    Ok(attempt_visits) => clear_visited(&mut map, attempt_visits.iter().copied()),
                    Err(attempt_visits) => {
                        count += 1;
                        clear_visited(&mut map, attempt_visits.iter().copied());
                    }
                }
                map.remove_obstacle(obs_pos);
            }
            count
        });
        children.push(child);
    }

    children
        .into_iter()
        .map(|child| child.join().unwrap())
        .sum()
}

#[inline]
fn mark_up(pos: u8) -> u8 {
    assert_ne!(pos, b'#');
    pos | 0b1
}

#[inline]
fn already_up(pos: u8) -> bool {
    pos & 0b1 == 0b1
}

#[inline]
fn mark_right(pos: u8) -> u8 {
    assert_ne!(pos, b'#');
    pos | 0b10
}

#[inline]
fn already_right(pos: u8) -> bool {
    pos & 0b10 == 0b10
}

#[inline]
fn mark_down(pos: u8) -> u8 {
    assert_ne!(pos, b'#');
    pos | 0b100
}

#[inline]
fn already_down(pos: u8) -> bool {
    pos & 0b100 == 0b100
}

#[inline]
fn mark_left(pos: u8) -> u8 {
    assert_ne!(pos, b'#');
    pos | 0b1000
}

#[inline]
fn already_left(pos: u8) -> bool {
    pos & 0b1000 == 0b1000
}

fn positions_visited(map: &mut Map, mut pos: Position) -> Result<Vec<Position>, Vec<Position>> {
    let mut positions = Vec::new();
    map.map[pos.0][pos.1] = mark_up(map.map[pos.0][pos.1]);
    positions.push(pos);
    loop {
        // going up
        while pos.0 > 0 && map.map[pos.0 - 1][pos.1] != b'#' {
            pos = (pos.0 - 1, pos.1);
            if already_up(map.map[pos.0][pos.1]) {
                return Err(positions);
            }
            map.map[pos.0][pos.1] = mark_up(map.map[pos.0][pos.1]);
            positions.push(pos);
        }
        if pos.0 == 0 {
            break;
        }
        // going right
        while pos.1 < map.width - 1 && map.map[pos.0][pos.1 + 1] != b'#' {
            pos = (pos.0, pos.1 + 1);
            if already_right(map.map[pos.0][pos.1]) {
                return Err(positions);
            }
            map.map[pos.0][pos.1] = mark_right(map.map[pos.0][pos.1]);
            positions.push(pos);
        }
        if pos.1 == map.width - 1 {
            break;
        }
        // going down
        while pos.0 < map.height - 1 && map.map[pos.0 + 1][pos.1] != b'#' {
            pos = (pos.0 + 1, pos.1);
            if already_down(map.map[pos.0][pos.1]) {
                return Err(positions);
            }
            map.map[pos.0][pos.1] = mark_down(map.map[pos.0][pos.1]);
            positions.push(pos);
        }
        if pos.0 == map.height - 1 {
            break;
        }
        // going left
        while pos.1 > 0 && map.map[pos.0][pos.1 - 1] != b'#' {
            pos = (pos.0, pos.1 - 1);
            if already_left(map.map[pos.0][pos.1]) {
                return Err(positions);
            }
            map.map[pos.0][pos.1] = mark_left(map.map[pos.0][pos.1]);
            positions.push(pos);
        }
        if pos.1 == 0 {
            break;
        }
    }
    Ok(positions)
}

#[cfg(test)]
mod tests {
    use super::{count_distinct_positions, count_loopable_obstacle_insertions};

    #[test]
    fn part1_example() {
        let result = count_distinct_positions("example.txt");
        assert_eq!(result, 41);
    }

    #[test]
    fn part1() {
        let result = count_distinct_positions("input.txt");
        assert_eq!(result, 4711);
    }

    #[test]
    fn part2_example() {
        let result = count_loopable_obstacle_insertions("example.txt", 1);
        assert_eq!(result, 6);
    }

    #[test]
    fn part2() {
        let result = count_loopable_obstacle_insertions("input.txt", 8);
        assert_eq!(result, 1562);
    }
}
