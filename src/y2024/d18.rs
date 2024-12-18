#![allow(dead_code)]

use std::{collections::BinaryHeap, fs};

#[derive(Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    steps: usize,
}
impl Position {
    fn new(x: usize, y: usize, steps: usize) -> Self {
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

fn find_shortest_path(maze: &[Vec<u8>], seen: &mut [Vec<usize>]) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let start_pos = Position::new(0, 0, 0);
    let dest_x = maze[0].len() - 1;
    let dest_y = maze.len() - 1;

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
        if pos.y > 0 && maze[pos.y - 1][pos.x] != b'#' && seen[pos.y - 1][pos.x] > pos.steps + 1 {
            queue.push(Position::new(pos.x, pos.y - 1, pos.steps + 1));
        }
        // right
        if pos.x < maze[0].len() - 1
            && maze[pos.y][pos.x + 1] != b'#'
            && seen[pos.y][pos.x + 1] > pos.steps + 1
        {
            queue.push(Position::new(pos.x + 1, pos.y, pos.steps + 1));
        }
        // down
        if pos.y < maze.len() - 1
            && maze[pos.y + 1][pos.x] != b'#'
            && seen[pos.y + 1][pos.x] > pos.steps + 1
        {
            queue.push(Position::new(pos.x, pos.y + 1, pos.steps + 1));
        }
        // left
        if pos.x > 0 && maze[pos.y][pos.x - 1] != b'#' && seen[pos.y][pos.x - 1] > pos.steps + 1 {
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

fn build_maze(positions: &[(usize, usize)], width: usize, height: usize) -> Vec<Vec<u8>> {
    let mut maze = vec![vec![b'.'; width]; height];
    for (x, y) in positions {
        maze[*y][*x] = b'#';
    }
    maze
}

fn shortest_path_through_corrupted_memory(
    filename: &str,
    map_width: usize,
    map_height: usize,
    ticks: usize,
) -> usize {
    let positions = load_input(filename);
    let maze = build_maze(&positions[..ticks], map_width, map_height);
    let mut seen = vec![vec![usize::MAX; maze[0].len()]; maze.len()];
    find_shortest_path(&maze, &mut seen).unwrap()
}

fn first_coord_to_block_exit(
    filename: &str,
    map_width: usize,
    map_height: usize,
    skip_ticks: usize,
) -> (usize, usize) {
    let positions = load_input(filename);
    let mut seen = vec![vec![usize::MAX; map_width]; map_height];
    let mut low = skip_ticks;
    let mut high = positions.len() - 1;
    let mut mid = (low + high) / 2;

    while low < high {
        let maze = build_maze(&positions[..=mid], map_width, map_height);
        seen.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|p| {
                *p = usize::MAX;
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
