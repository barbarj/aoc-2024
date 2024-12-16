#![allow(dead_code)]
use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

fn get_maze(filename: &str) -> Vec<Vec<u8>> {
    let contents = fs::read_to_string("input/2024/16/".to_owned() + filename).unwrap();
    contents
        .lines()
        .map(|line| line.bytes().collect())
        .collect()
}

#[derive(Eq, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
}

#[derive(Eq, PartialEq)]
struct Position {
    x: i64,
    y: i64,
    score: usize,
    weight: usize,
    direction: Direction,
}
impl Position {
    fn new(x: i64, y: i64, score: usize, weight: usize, direction: Direction) -> Self {
        Position {
            x,
            y,
            score,
            weight,
            direction,
        }
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_path(maze: &[Vec<u8>], pos: (i64, i64)) -> usize {
    let mut queue = BinaryHeap::new();
    let mut seen = vec![vec![usize::MAX; maze[0].len()]; maze.len()];
    let start_pos = Position::new(pos.0, pos.1, 0, 0, Direction::East);
    queue.push(start_pos);
    while let Some(pos) = queue.pop() {
        if maze[pos.y as usize][pos.x as usize] == b'E' {
            return pos.score;
        }
        if seen[pos.y as usize][pos.x as usize] < pos.score {
            continue;
        }
        seen[pos.y as usize][pos.x as usize] = pos.score;
        let (dx, dy, lx, ly, rx, ry) = match pos.direction {
            Direction::North => (0, -1, -1, 0, 1, 0),
            Direction::East => (1, 0, 0, -1, 0, 1),
            Direction::South => (0, 1, 1, 0, -1, 0),
            Direction::West => (-1, 0, 0, 1, 0, -1),
        };
        if maze[(pos.y + dy) as usize][(pos.x + dx) as usize] != b'#' {
            queue.push(Position::new(
                pos.x + dx,
                pos.y + dy,
                pos.score + 1,
                0,
                pos.direction.clone(),
            ));
        }
        if maze[(pos.y + ry) as usize][(pos.x + rx) as usize] != b'#' {
            queue.push(Position::new(
                pos.x + rx,
                pos.y + ry,
                pos.score + 1001,
                0,
                pos.direction.right(),
            ));
        }
        if maze[(pos.y + ly) as usize][(pos.x + lx) as usize] != b'#' {
            queue.push(Position::new(
                pos.x + lx,
                pos.y + ly,
                pos.score + 1001,
                0,
                pos.direction.left(),
            ));
        }
    }
    unreachable!();
}

pub fn best_maze_score(filename: &str) -> usize {
    let maze = get_maze(filename);
    let starting_pos = (1i64, (maze.len() - 2) as i64);
    find_shortest_path(&maze, starting_pos)
}

fn build_best_paths(
    maze: &[Vec<u8>],
    starting_pos: (i64, i64),
    best_score: usize,
) -> HashSet<(i64, i64)> {
    let mut stack = Vec::new();
    // (weight, x, y)
    let mut path: Vec<(usize, i64, i64)> = Vec::new();
    let mut seen = vec![vec![usize::MAX; maze[0].len()]; maze.len()];
    let mut best_tiles = HashSet::new();
    stack.push(Position::new(
        starting_pos.0,
        starting_pos.1,
        0,
        0,
        Direction::East,
    ));
    while let Some(pos) = stack.pop() {
        while !path.is_empty() && path.last().unwrap().0 >= pos.weight {
            path.pop();
        }
        if pos.score > best_score {
            continue;
        }
        if maze[pos.y as usize][pos.x as usize] == b'E' {
            if pos.score == best_score {
                best_tiles.extend(path.iter().map(|v| (v.1, v.2)));
                best_tiles.insert((pos.x, pos.y));
            }
            continue;
        }
        let sv = &mut seen[pos.y as usize][pos.x as usize];
        if *sv < pos.score && pos.score >= 1000 && *sv != pos.score - 1000 {
            continue;
        }
        *sv = (*sv).min(pos.score);
        path.push((pos.weight, pos.x, pos.y));
        let (dx, dy, lx, ly, rx, ry) = match pos.direction {
            Direction::North => (0, -1, -1, 0, 1, 0),
            Direction::East => (1, 0, 0, -1, 0, 1),
            Direction::South => (0, 1, 1, 0, -1, 0),
            Direction::West => (-1, 0, 0, 1, 0, -1),
        };
        if maze[(pos.y + dy) as usize][(pos.x + dx) as usize] != b'#' {
            stack.push(Position::new(
                pos.x + dx,
                pos.y + dy,
                pos.score + 1,
                pos.weight + 1,
                pos.direction.clone(),
            ));
        }
        if maze[(pos.y + ry) as usize][(pos.x + rx) as usize] != b'#' {
            stack.push(Position::new(
                pos.x + rx,
                pos.y + ry,
                pos.score + 1001,
                pos.weight + 1,
                pos.direction.right(),
            ));
        }
        if maze[(pos.y + ly) as usize][(pos.x + lx) as usize] != b'#' {
            stack.push(Position::new(
                pos.x + lx,
                pos.y + ly,
                pos.score + 1001,
                pos.weight + 1,
                pos.direction.left(),
            ));
        }
    }
    best_tiles
}

fn show_best_tiles(maze: &mut [Vec<u8>], best_tiles: impl Iterator<Item = (i64, i64)>) {
    for (x, y) in best_tiles {
        maze[y as usize][x as usize] = b'O';
    }
    for line in maze {
        let s: String = line
            .iter()
            .map(|c| char::from_u32(*c as u32).unwrap())
            .collect();
        println!("{s}");
    }
}

fn count_tiles_on_best_paths(filename: &str) -> usize {
    let maze = get_maze(filename);
    let starting_pos = (1, (maze.len() - 2) as i64);
    let best_tiles = build_best_paths(&maze, starting_pos, find_shortest_path(&maze, starting_pos));
    best_tiles.len()
}

#[cfg(test)]
mod tests {
    use super::{best_maze_score, count_tiles_on_best_paths};

    #[test]
    fn part1_small_example() {
        let result = best_maze_score("small_example.txt");
        assert_eq!(result, 7036);
    }

    #[test]
    fn part1_example() {
        let result = best_maze_score("example.txt");
        assert_eq!(result, 11048);
    }

    #[test]
    fn part1() {
        let result = best_maze_score("input.txt");
        assert_eq!(result, 109516);
    }

    #[test]
    fn part2_small_example() {
        let result = count_tiles_on_best_paths("small_example.txt");
        assert_eq!(result, 45);
    }

    #[test]
    fn part2_example() {
        let result = count_tiles_on_best_paths("example.txt");
        assert_eq!(result, 64);
    }

    #[test]
    fn part2() {
        let result = count_tiles_on_best_paths("input.txt");
        assert_eq!(result, 568);
    }
}
