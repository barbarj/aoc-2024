#![allow(dead_code)]
use std::{fs, iter, thread::sleep, time::Duration};

use itertools::Itertools;
use regex::Regex;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Position {
    x: i64,
    y: i64,
}
impl Position {
    fn new(x: i64, y: i64) -> Self {
        Position { x, y }
    }

    fn from_str(x: &str, y: &str) -> Self {
        Self::new(x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
    }

    fn translate_by_pos(&mut self, pos: &Self) {
        self.x += pos.x;
        self.y += pos.y;
    }
}

fn load_input(filename: &str) -> (Vec<Position>, Vec<Position>) {
    let contents = fs::read_to_string("input/14/".to_owned() + filename).unwrap();
    let line_pattern = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    contents
        .lines()
        .map(|line| {
            let captures = line_pattern.captures(line).unwrap();
            let position = Position::from_str(&captures[1], &captures[2]);
            let velocity = Position::from_str(&captures[3], &captures[4]);
            (position, velocity)
        })
        .collect()
}

fn positions_after_steps<'a>(
    starting_positions: &'a [Position],
    velocities: &'a [Position],
    width: i64,
    height: i64,
    steps: i64,
) -> impl Iterator<Item = Position> + 'a {
    starting_positions
        .iter()
        .zip(velocities.iter())
        .map(move |(pos, velocity)| {
            let new_x = (pos.x + (steps * velocity.x)).rem_euclid(width);
            let new_y = (pos.y + (steps * velocity.y)).rem_euclid(height);
            Position::new(new_x, new_y)
        })
}

fn part1_score(filename: &str, width: i64, height: i64, steps: i64) -> i64 {
    assert!(width % 2 == 1);
    assert!(height % 2 == 1);

    let (starting_positions, velocities) = load_input(filename);
    let mut quandrant_counts = [0, 0, 0, 0];
    for pos in positions_after_steps(&starting_positions, &velocities, width, height, steps) {
        if pos.x < width / 2 && pos.y < height / 2 {
            quandrant_counts[0] += 1;
        } else if pos.x > width / 2 && pos.y < height / 2 {
            quandrant_counts[1] += 1;
        } else if pos.x > width / 2 && pos.y > height / 2 {
            quandrant_counts[2] += 1;
        } else if pos.x < width / 2 && pos.y > height / 2 {
            quandrant_counts[3] += 1;
        }
    }
    quandrant_counts.iter().product()
}

fn render_grid(positions: &[Position], width: i64, height: i64, steps: i64) {
    // clear screen
    println!("======");
    println!("| {steps} |");
    println!("======");
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    for pos in positions {
        grid[pos.y as usize][pos.x as usize] = 'X';
    }
    for (idx, row) in grid.into_iter().enumerate() {
        let mut line: String = row
            .iter()
            .interleave(iter::repeat(&' ').take(row.len()))
            .collect();
        if idx == 0 || idx == 2 {
            line.push_str("======");
        }
        if idx == 1 {
            line.push_str(&format!("| {steps} |"));
        }

        println!("{line}");
    }
}

fn increment_positions(
    positions: &mut [Position],
    velocities: &[Position],
    width: i64,
    height: i64,
) {
    for (pos, vel) in positions.iter_mut().zip(velocities.iter()) {
        let new_x = (pos.x + vel.x).rem_euclid(width);
        let new_y = (pos.y + vel.y).rem_euclid(height);
        *pos = Position::new(new_x, new_y)
    }
}

fn clear_screen() {
    print!("\x1B[2J");
}

fn in_center_up(pos: &Position, width: i64, height: i64) -> bool {
    pos.x >= width / 4 && pos.x <= width * 3 / 4 && pos.y < height / 2
}

fn part2_estimate(filename: &str, width: i64, height: i64) -> i64 {
    let (mut positions, velocities) = load_input(filename);
    let threshold = positions.len() / 2;
    let mut ticks = 0;
    while ticks < 1000000000 {
        increment_positions(&mut positions, &velocities, width, height);
        ticks += 1;
        if positions
            .iter()
            .filter(|pos| in_center_up(pos, width, height))
            .count()
            >= threshold
        {
            break;
        }
    }
    ticks
}

pub fn part2_display(filename: &str, width: i64, height: i64, tick: i64) {
    let (positions, velocities) = load_input(filename);
    let positions: Vec<Position> =
        positions_after_steps(&positions, &velocities, width, height, tick).collect();
    render_grid(&positions, width, height, tick);
}

pub fn part2_animate(filename: &str, width: i64, height: i64, start_at: i64) {
    let (positions, velocities) = load_input(filename);
    let mut positions: Vec<Position> =
        positions_after_steps(&positions, &velocities, width, height, start_at).collect();
    let mut steps = start_at;
    loop {
        clear_screen();
        render_grid(&positions, width, height, steps);
        increment_positions(&mut positions, &velocities, width, height);
        steps += 1;
        sleep(Duration::from_millis(1000));
    }
}

#[cfg(test)]
mod tests {
    use crate::d14::{part1_score, part2_estimate};

    #[test]
    fn part1_example() {
        let result = part1_score("example.txt", 11, 7, 100);
        assert_eq!(result, 12);
    }

    #[test]
    fn part1() {
        let result = part1_score("input.txt", 101, 103, 100);
        assert_eq!(result, 215987200);
    }

    #[test]
    fn part2() {
        let result = part2_estimate("input.txt", 101, 103);
        assert_eq!(result, 8050);
    }
}
