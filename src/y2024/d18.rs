#![allow(dead_code)]

use std::{collections::BinaryHeap, fs};

#[derive(Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    steps: u16,
}
impl Position {
    fn new(x: usize, y: usize, steps: u16) -> Self {
        Position { x, y, steps }
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.cmp(&other.steps).reverse()
    }
}
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    maze: Vec<u128>,
    width: usize,
    height: usize,
}
impl Maze {
    fn new(width: usize, height: usize) -> Self {
        Maze {
            maze: vec![0u128; height],
            width,
            height,
        }
    }

    fn set_pos(&mut self, x: usize, y: usize) {
        self.maze[y] |= 1 << x;
    }

    fn get_pos(&self, x: usize, y: usize) -> bool {
        self.maze[y] & (1 << x) > 0
    }
}

fn find_shortest_path(maze: &Maze, seen: &mut [Vec<u16>]) -> Option<u16> {
    let mut queue = BinaryHeap::new();
    let start_pos = Position::new(0, 0, 0);
    let dest_x = maze.width - 1;
    let dest_y = maze.height - 1;

    queue.push(start_pos);
    while let Some(pos) = queue.pop() {
        if pos.x == dest_x && pos.y == dest_y {
            return Some(pos.steps);
        }
        if seen[pos.y][pos.x] <= pos.steps {
            continue;
        }
        seen[pos.y][pos.x] = pos.steps;
        // up
        if pos.y > 0 && !maze.get_pos(pos.x, pos.y - 1) && seen[pos.y - 1][pos.x] > pos.steps + 1 {
            queue.push(Position::new(pos.x, pos.y - 1, pos.steps + 1));
        }
        // right
        if pos.x < maze.width - 1
            && !maze.get_pos(pos.x + 1, pos.y)
            && seen[pos.y][pos.x + 1] > pos.steps + 1
        {
            queue.push(Position::new(pos.x + 1, pos.y, pos.steps + 1));
        }
        // down
        if pos.y < maze.height - 1
            && !maze.get_pos(pos.x, pos.y + 1)
            && seen[pos.y + 1][pos.x] > pos.steps + 1
        {
            queue.push(Position::new(pos.x, pos.y + 1, pos.steps + 1));
        }
        // left
        if pos.x > 0 && !maze.get_pos(pos.x - 1, pos.y) && seen[pos.y][pos.x - 1] > pos.steps + 1 {
            queue.push(Position::new(pos.x - 1, pos.y, pos.steps + 1));
        }
    }
    None
}

fn load_input(filename: &str) -> Vec<(usize, usize)> {
    let contents = fs::read_to_string("input/2024/18/".to_owned() + filename).unwrap();
    contents
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

fn build_maze(positions: &[(usize, usize)], width: usize, height: usize) -> Maze {
    let mut maze = Maze::new(width, height);
    for (x, y) in positions {
        maze.set_pos(*x, *y);
    }
    maze
}

fn shortest_path_through_corrupted_memory(
    filename: &str,
    map_width: usize,
    map_height: usize,
    ticks: usize,
) -> u16 {
    let positions = load_input(filename);
    let maze = build_maze(&positions[..ticks], map_width, map_height);
    let mut seen = vec![vec![u16::MAX; map_width]; map_height];
    find_shortest_path(&maze, &mut seen).unwrap()
}

fn first_coord_to_block_exit(
    filename: &str,
    map_width: usize,
    map_height: usize,
    skip_ticks: usize,
) -> (usize, usize) {
    let positions = load_input(filename);
    let mut seen = vec![vec![u16::MAX; map_width]; map_height];
    let mut low = skip_ticks;
    let mut high = positions.len() - 1;
    let mut mid = (low + high) / 2;

    while low < high {
        let maze = build_maze(&positions[..=mid], map_width, map_height);
        seen.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|p| {
                *p = u16::MAX;
            })
        });
        if find_shortest_path(&maze, &mut seen).is_none() {
            high = mid;
        } else {
            low = mid + 1;
        }
        mid = (low + high) / 2;
    }
    positions[mid]
}

#[cfg(test)]
mod tests {
    use super::{first_coord_to_block_exit, shortest_path_through_corrupted_memory};

    #[test]
    fn part1_example() {
        let result = shortest_path_through_corrupted_memory("example.txt", 7, 7, 12);
        assert_eq!(result, 22);
    }

    #[test]
    fn part1() {
        let result = shortest_path_through_corrupted_memory("input.txt", 71, 71, 1024);
        assert_eq!(result, 354);
    }

    #[test]
    fn part2_example() {
        let result = first_coord_to_block_exit("example.txt", 7, 7, 12);
        assert_eq!(result, (6, 1));
    }

    #[test]
    fn part2() {
        let result = first_coord_to_block_exit("input.txt", 71, 71, 1024);
        assert_eq!(result, (36, 17));
    }
}
