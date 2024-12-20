#![allow(dead_code)]
use std::fs;

use itertools::Itertools;

struct Maze<T: Copy> {
    maze: Vec<T>,
    width: usize,
    height: usize,
}
impl<T: Copy> Maze<T> {
    fn get(&self, x: i64, y: i64) -> T {
        let idx = (y as usize * self.width) + x as usize;
        self.maze[idx]
    }

    fn set(&mut self, x: i64, y: i64, val: T) {
        let idx = (y as usize * self.width) + x as usize;
        self.maze[idx] = val;
    }
}

fn load_maze(filename: &str) -> Maze<u8> {
    let contents = fs::read_to_string("input/2024/20/".to_owned() + filename).unwrap();
    let mut width = 0;
    let mut height = 0;
    let mut maze = Vec::new();
    for line in contents.lines() {
        width = line.len();
        height += 1;
        maze.extend(line.bytes());
    }
    Maze {
        maze,
        width,
        height,
    }
}

fn walk_maze(maze: &Maze<u8>, seen: &mut Maze<usize>) -> Vec<(i64, i64)> {
    let start_idx = maze.maze.iter().find_position(|c| **c == b'S').unwrap().0;
    let (x, y) = (start_idx % maze.width, start_idx / maze.width);
    let (mut x, mut y) = (x as i64, y as i64);
    let mut walked = Vec::new();
    let mut steps = 0;
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    while maze.get(x, y) != b'E' {
        seen.set(x, y, steps);
        walked.push((x, y));
        for (dx, dy) in directions {
            if (maze.get(x + dx, y + dy) == b'.' && seen.get(x + dx, y + dy) == usize::MAX)
                || maze.get(x + dx, y + dy) == b'E'
            {
                x += dx;
                y += dy;
                steps += 1;
                break;
            }
        }
    }
    seen.set(x, y, steps);
    walked.push((x, y));
    walked
}

fn shortcuts_that_save_over_x(filename: &str, cheat_len: i64, min_saved: usize) -> usize {
    let maze = load_maze(filename);
    let mut seen = Maze {
        maze: vec![usize::MAX; maze.width * maze.height],
        width: maze.width,
        height: maze.height,
    };

    //walk maze once
    let walked = walk_maze(&maze, &mut seen);

    // now do shortcuts
    let mut qualifying_count = 0;
    for (x, y) in walked {
        qualifying_count += process_pos(&maze, &seen, cheat_len, x, y, min_saved);
    }

    qualifying_count
}

struct CheatState {
    state: [Vec<usize>; 20],
}
impl CheatState {
    fn new() -> Self {
        CheatState {
            state: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
        }
    }

    fn at_steps_left(&self, left: usize) -> &[usize] {
        assert!(left < 20);
        &self.state[left]
    }

    fn insert(&mut self, left: usize, saved: usize) {
        assert!(left < 20);
        self.state[left].push(saved);
    }
}

fn distance(x1: i64, y1: i64, x2: i64, y2: i64) -> usize {
    ((x2 - x1).abs() + (y2 - y1).abs()).try_into().unwrap()
}

fn process_pos(
    maze: &Maze<u8>,
    seen: &Maze<usize>,
    cheat_len: i64,
    x: i64,
    y: i64,
    min_saved: usize,
) -> usize {
    let mw = maze.width as i64 - 1;
    let mh = maze.height as i64 - 1;
    let x_range = 0.max(x - cheat_len)..=mw.min(x + cheat_len);
    let y_range = 0.max(y - cheat_len)..=mh.min(y + cheat_len);
    x_range
        .flat_map(|x| y_range.clone().map(move |y| (x, y)))
        .filter(|(px, py)| {
            let dist = distance(x, y, *px, *py);
            maze.get(*px, *py) != b'#'
                && dist <= cheat_len as usize
                && seen.get(*px, *py) > seen.get(x, y) + dist
                && seen.get(*px, *py) - (seen.get(x, y) + dist) >= min_saved
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::shortcuts_that_save_over_x;

    #[test]
    fn part1_example() {
        let result = shortcuts_that_save_over_x("example.txt", 2, 20);
        assert_eq!(result, 5);
    }

    #[test]
    fn part1() {
        let result = shortcuts_that_save_over_x("input.txt", 2, 100);
        assert_eq!(result, 1307);
    }

    #[test]
    fn part2_example_70() {
        let result = shortcuts_that_save_over_x("example.txt", 20, 70);
        assert_eq!(result, 41);
    }

    #[test]
    fn part2_example_76() {
        let result = shortcuts_that_save_over_x("example.txt", 20, 76);
        assert_eq!(result, 3);
    }

    #[test]
    fn part2() {
        let result = shortcuts_that_save_over_x("input.txt", 20, 100);
        assert_eq!(result, 986545);
    }
}
