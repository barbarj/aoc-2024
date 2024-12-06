#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

struct MapData {
    obstacles_by_row: HashMap<usize, Vec<usize>>,
    obstacles_by_col: HashMap<usize, Vec<usize>>,
    width: usize,
    height: usize,
}
impl MapData {
    fn new() -> Self {
        MapData {
            obstacles_by_row: HashMap::new(),
            obstacles_by_col: HashMap::new(),
            width: 0,
            height: 0,
        }
    }

    fn add_obstacle(&mut self, (row, col): (usize, usize)) {
        self.obstacles_by_row
            .entry(row)
            .and_modify(|v: &mut Vec<usize>| v.push(col))
            .or_insert(vec![col]);
        self.obstacles_by_col
            .entry(col)
            .and_modify(|v: &mut Vec<usize>| v.push(row))
            .or_insert(vec![row]);
    }

    fn remove_obstacle(&mut self, (row, col): (usize, usize)) {
        self.obstacles_by_row
            .entry(row)
            .and_modify(|v: &mut Vec<usize>| {
                if let Some(pos) = v.iter().enumerate().find(|(_, x)| **x == col).map(|x| x.0) {
                    v.remove(pos);
                }
            });
        self.obstacles_by_col
            .entry(col)
            .and_modify(|v: &mut Vec<usize>| {
                if let Some(pos) = v.iter().enumerate().find(|(_, x)| **x == row).map(|x| x.0) {
                    v.remove(pos);
                }
            });
    }
}

fn get_input(filename: &str) -> (MapData, (usize, usize)) {
    let file = File::open("input/6/".to_owned() + filename).unwrap();

    let mut guard_pos = None;
    let mut map_data = MapData::new();
    for (row, line) in BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        if map_data.width == 0 {
            map_data.width = line.len();
        }
        map_data.height += 1;
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                map_data.add_obstacle((row, col));
            } else if char == '^' {
                guard_pos = Some((row, col));
            }
        }
    }
    assert!(map_data.width > 0);
    assert!(map_data.height > 0);
    (map_data, guard_pos.unwrap())
}

fn count_distinct_positions(filename: &str) -> usize {
    let (map_data, starting_pos) = get_input(filename);
    let visited = positions_visited(&map_data, starting_pos).unwrap();
    visited.union_all().len()
}

fn count_loopable_obstacle_insertions(filename: &str) -> usize {
    let (mut map_data, starting_pos) = get_input(filename);
    let visited = positions_visited(&map_data, starting_pos).unwrap();
    let mut count = 0;
    for (row, col) in visited.union_all() {
        let obs_pos = (row, col);
        map_data.add_obstacle(obs_pos);
        if positions_visited(&map_data, starting_pos).is_none() {
            count += 1;
        }
        map_data.remove_obstacle(obs_pos);
    }
    count
}

struct PositionsVisited {
    up: HashSet<(usize, usize)>,
    right: HashSet<(usize, usize)>,
    down: HashSet<(usize, usize)>,
    left: HashSet<(usize, usize)>,
}
impl PositionsVisited {
    fn union_all(self) -> HashSet<(usize, usize)> {
        let mut all = self.up;
        all.extend(self.right);
        all.extend(self.down);
        all.extend(self.left);
        all
    }
}

fn positions_visited(map_data: &MapData, mut pos: (usize, usize)) -> Option<PositionsVisited> {
    let mut up_positions = HashSet::new();
    let mut right_positions = HashSet::new();
    let mut down_positions = HashSet::new();
    let mut left_positions = HashSet::new();
    loop {
        // going up
        if let Some(next_obstacle_row) = map_data
            .obstacles_by_col
            .get(&pos.1)
            .and_then(|obs| obs.iter().filter(|r| **r < pos.0).max())
        {
            let visited_range = *next_obstacle_row + 1..=pos.0;
            if visited_range
                .clone()
                .any(|r| up_positions.contains(&(r, pos.1)))
            {
                return None;
            }
            up_positions.extend(visited_range.map(|r| (r, pos.1)));
            pos = (next_obstacle_row + 1, pos.1);
        } else {
            up_positions.extend((0..=pos.0).map(|r| (r, pos.1)));
            break;
        }
        // going right
        if let Some(next_obstacle_col) = map_data
            .obstacles_by_row
            .get(&pos.0)
            .and_then(|obs| obs.iter().filter(|c| **c > pos.1).min())
        {
            let visited_range = pos.1..*next_obstacle_col;
            if visited_range
                .clone()
                .any(|c| right_positions.contains(&(pos.0, c)))
            {
                return None;
            }
            right_positions.extend(visited_range.map(|c| (pos.0, c)));
            pos = (pos.0, next_obstacle_col - 1);
        } else {
            right_positions.extend((pos.1..map_data.width).map(|c| (pos.0, c)));
            break;
        }
        // going down
        if let Some(next_obstacle_row) = map_data
            .obstacles_by_col
            .get(&pos.1)
            .and_then(|obs| obs.iter().filter(|r| **r > pos.0).min())
        {
            let visited_range = pos.0..*next_obstacle_row;
            if visited_range
                .clone()
                .any(|r| down_positions.contains(&(r, pos.1)))
            {
                return None;
            }
            down_positions.extend(visited_range.map(|r| (r, pos.1)));
            pos = (next_obstacle_row - 1, pos.1);
        } else {
            down_positions.extend((pos.0..map_data.height).map(|r| (r, pos.1)));
            break;
        }
        // going left
        if let Some(next_obstacle_col) = map_data
            .obstacles_by_row
            .get(&pos.0)
            .and_then(|obs| obs.iter().filter(|c| **c < pos.1).max())
        {
            let visited_range = *next_obstacle_col + 1..=pos.1;
            if visited_range
                .clone()
                .any(|c| left_positions.contains(&(pos.0, c)))
            {
                return None;
            }
            left_positions.extend(visited_range.map(|c| (pos.0, c)));
            pos = (pos.0, next_obstacle_col + 1);
        } else {
            left_positions.extend((0..=pos.1).map(|c| (pos.0, c)));
            break;
        }
    }

    Some(PositionsVisited {
        up: up_positions,
        right: right_positions,
        down: down_positions,
        left: left_positions,
    })
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
        let result = count_loopable_obstacle_insertions("example.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn part2() {
        let result = count_loopable_obstacle_insertions("input.txt");
        assert_eq!(result, 1562);
    }
}
