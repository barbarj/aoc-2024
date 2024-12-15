#![allow(dead_code)]
use std::{char, collections::VecDeque, fs};

struct TwoDArray<T: Clone> {
    arr: Vec<T>,
    width: usize,
    height: usize,
}
impl<T: Clone> TwoDArray<T> {
    fn new(arr: Vec<T>, width: usize, height: usize) -> Self {
        TwoDArray { arr, width, height }
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        let idx = (self.width * y) + x;
        self.arr[idx] = val;
    }

    fn get(&self, x: usize, y: usize) -> T {
        let idx = (self.width * y) + x;
        self.arr[idx].clone()
    }
}

struct Map {
    map: TwoDArray<Space>,
    robot_pos: (usize, usize),
}
impl Map {
    fn new(mut input: Vec<Space>, width: usize, height: usize) -> Self {
        let robot_idx = input
            .iter()
            .enumerate()
            .find(|(_, c)| **c == Space::Robot)
            .unwrap()
            .0;
        input[robot_idx] = Space::Free;
        let robot_pos = (robot_idx % width, robot_idx / width);
        Map {
            map: TwoDArray::new(input, width, height),
            robot_pos,
        }
    }

    fn up(&mut self) {
        // if no resitance, just move
        if self.map.get(self.robot_pos.0, self.robot_pos.1 - 1) == Space::Free {
            self.robot_pos = (self.robot_pos.0, self.robot_pos.1 - 1);
            return;
        }
        // walk boxes
        let mut i = 1;
        while self.map.get(self.robot_pos.0, self.robot_pos.1 - i) == Space::Box {
            i += 1;
        }
        // if at wall, hence no free space, do nothing
        if self.map.get(self.robot_pos.0, self.robot_pos.1 - i) == Space::Wall {
            return;
        }

        // move boxes
        while i > 0 {
            self.map.set(
                self.robot_pos.0,
                self.robot_pos.1 - i,
                self.map.get(self.robot_pos.0, self.robot_pos.1 - i + 1),
            );
            i -= 1;
        }
        // move robot
        self.robot_pos = (self.robot_pos.0, self.robot_pos.1 - 1);
        self.map
            .set(self.robot_pos.0, self.robot_pos.1, Space::Free)
    }

    fn down(&mut self) {
        // if no resitance, just move
        if self.map.get(self.robot_pos.0, self.robot_pos.1 + 1) == Space::Free {
            self.robot_pos = (self.robot_pos.0, self.robot_pos.1 + 1);
            return;
        }
        // walk boxes
        let mut i = 1;
        while self.map.get(self.robot_pos.0, self.robot_pos.1 + i) == Space::Box {
            i += 1;
        }
        // if at wall, hence no free space, do nothing
        if self.map.get(self.robot_pos.0, self.robot_pos.1 + i) == Space::Wall {
            return;
        }

        // move boxes
        while i > 0 {
            self.map.set(
                self.robot_pos.0,
                self.robot_pos.1 + i,
                self.map.get(self.robot_pos.0, self.robot_pos.1 + i - 1),
            );
            i -= 1;
        }
        // move robot
        self.robot_pos = (self.robot_pos.0, self.robot_pos.1 + 1);
        self.map
            .set(self.robot_pos.0, self.robot_pos.1, Space::Free);
    }

    fn right(&mut self) {
        // if no resitance, just move
        if self.map.get(self.robot_pos.0 + 1, self.robot_pos.1) == Space::Free {
            self.robot_pos = (self.robot_pos.0 + 1, self.robot_pos.1);
            return;
        }
        // walk boxes
        let mut i = 1;
        while self.map.get(self.robot_pos.0 + i, self.robot_pos.1) == Space::Box {
            i += 1;
        }
        // if at wall, hence no free space, do nothing
        if self.map.get(self.robot_pos.0 + i, self.robot_pos.1) == Space::Wall {
            return;
        }

        // move boxes
        while i > 0 {
            self.map.set(
                self.robot_pos.0 + i,
                self.robot_pos.1,
                self.map.get(self.robot_pos.0 + i - 1, self.robot_pos.1),
            );
            i -= 1;
        }
        // move robot
        self.robot_pos = (self.robot_pos.0 + 1, self.robot_pos.1);
        self.map
            .set(self.robot_pos.0, self.robot_pos.1, Space::Free);
    }

    fn left(&mut self) {
        // if no resitance, just move
        if self.map.get(self.robot_pos.0 - 1, self.robot_pos.1) == Space::Free {
            self.robot_pos = (self.robot_pos.0 - 1, self.robot_pos.1);
            return;
        }
        // walk boxes
        let mut i = 1;
        while self.map.get(self.robot_pos.0 - i, self.robot_pos.1) == Space::Box {
            i += 1;
        }
        // if at wall, hence no free space, do nothing
        if self.map.get(self.robot_pos.0 - i, self.robot_pos.1) == Space::Wall {
            return;
        }

        // move boxes
        while i > 0 {
            self.map.set(
                self.robot_pos.0 - i,
                self.robot_pos.1,
                self.map.get(self.robot_pos.0 - i + 1, self.robot_pos.1),
            );
            i -= 1;
        }
        // move robot
        self.robot_pos = (self.robot_pos.0 - 1, self.robot_pos.1);
        self.map
            .set(self.robot_pos.0, self.robot_pos.1, Space::Free);
    }

    fn wide_up(&mut self, seen: &mut TwoDArray<bool>) {
        if self.map.get(self.robot_pos.0, self.robot_pos.1 - 1) == Space::Free {
            self.robot_pos = (self.robot_pos.0, self.robot_pos.1 - 1);
            return;
        }

        // build graph of connected boxes
        let mut positions = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((self.robot_pos.0, self.robot_pos.1 - 1));
        while let Some((x, y)) = queue.pop_front() {
            if seen.get(x, y) {
                continue;
            }
            seen.set(x, y, true);
            if self.map.get(x, y) == Space::BoxLeft {
                positions.push((x, y));
                queue.push_back((x + 1, y)); // go right
                queue.push_back((x, y - 1)) // go up
            } else if self.map.get(x, y) == Space::BoxRight {
                positions.push((x, y));
                queue.push_back((x - 1, y)); // go left
                queue.push_back((x, y - 1)) // go up
            } else if self.map.get(x, y) == Space::Wall {
                for p in seen.arr.iter_mut() {
                    *p = false;
                }
                return; // stop, no movement possible
            }
        }
        // sort by y-value, so we can safely copy up
        positions.sort_unstable_by_key(|pos| pos.1);
        // now move each position up
        for (x, y) in positions {
            self.map.set(x, y - 1, self.map.get(x, y));
            self.map.set(x, y, Space::Free);
        }
        self.robot_pos = (self.robot_pos.0, self.robot_pos.1 - 1);
        for p in seen.arr.iter_mut() {
            *p = false;
        }
    }

    fn wide_down(&mut self, seen: &mut TwoDArray<bool>) {
        if self.map.get(self.robot_pos.0, self.robot_pos.1 + 1) == Space::Free {
            self.robot_pos = (self.robot_pos.0, self.robot_pos.1 + 1);
            return;
        }

        // build graph of connected boxes
        let mut positions = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((self.robot_pos.0, self.robot_pos.1 + 1));
        while let Some((x, y)) = queue.pop_front() {
            if seen.get(x, y) {
                continue;
            }
            seen.set(x, y, true);
            if self.map.get(x, y) == Space::BoxLeft {
                positions.push((x, y));
                queue.push_back((x + 1, y)); // go right
                queue.push_back((x, y + 1)) // go down
            } else if self.map.get(x, y) == Space::BoxRight {
                positions.push((x, y));
                queue.push_back((x - 1, y)); // go left
                queue.push_back((x, y + 1)) // go down
            } else if self.map.get(x, y) == Space::Wall {
                for p in seen.arr.iter_mut() {
                    *p = false;
                }
                return; // stop, no movement possible
            }
        }
        // sort by y-value, so we can safely copy up
        positions.sort_unstable_by_key(|pos| pos.1);
        positions.reverse();
        // now move each position down
        for (x, y) in positions {
            self.map.set(x, y + 1, self.map.get(x, y));
            self.map.set(x, y, Space::Free);
        }
        self.robot_pos = (self.robot_pos.0, self.robot_pos.1 + 1);
        for p in seen.arr.iter_mut() {
            *p = false;
        }
    }

    fn wide_right(&mut self, seen: &mut TwoDArray<bool>) {
        if self.map.get(self.robot_pos.0 + 1, self.robot_pos.1) == Space::Free {
            self.robot_pos = (self.robot_pos.0 + 1, self.robot_pos.1);
            return;
        }

        // build graph of connected boxes
        let mut positions = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((self.robot_pos.0 + 1, self.robot_pos.1));
        while let Some((x, y)) = queue.pop_front() {
            if seen.get(x, y) {
                continue;
            }
            seen.set(x, y, true);
            if self.map.get(x, y) == Space::BoxLeft {
                positions.push((x, y));
                positions.push((x + 1, y));
                queue.push_back((x + 2, y)); // go right
            } else if self.map.get(x, y) == Space::Wall {
                for p in seen.arr.iter_mut() {
                    *p = false;
                }
                return; // stop, no movement possible
            }
        }
        // sort by x-value, so we can safely copy right
        positions.sort_unstable_by_key(|pos| pos.0);
        positions.reverse();

        // now move each position right
        for (x, y) in positions {
            self.map.set(x + 1, y, self.map.get(x, y));
            self.map.set(x, y, Space::Free);
        }
        self.robot_pos = (self.robot_pos.0 + 1, self.robot_pos.1);
        for p in seen.arr.iter_mut() {
            *p = false;
        }
    }

    fn wide_left(&mut self, seen: &mut TwoDArray<bool>) {
        if self.map.get(self.robot_pos.0 - 1, self.robot_pos.1) == Space::Free {
            self.robot_pos = (self.robot_pos.0 - 1, self.robot_pos.1);
            return;
        }

        // build graph of connected boxes
        let mut positions = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((self.robot_pos.0 - 1, self.robot_pos.1));
        while let Some((x, y)) = queue.pop_front() {
            if seen.get(x, y) {
                continue;
            }
            seen.set(x, y, true);
            if self.map.get(x, y) == Space::BoxRight {
                positions.push((x, y));
                positions.push((x - 1, y));
                queue.push_back((x - 2, y)); // go left
            } else if self.map.get(x, y) == Space::Wall {
                for p in seen.arr.iter_mut() {
                    *p = false;
                }
                return; // stop, no movement possible
            }
        }
        // sort by x-value, so we can safely copy left
        positions.sort_unstable_by_key(|pos| pos.0);

        // now move each position left
        for (x, y) in positions {
            self.map.set(x - 1, y, self.map.get(x, y));
            self.map.set(x, y, Space::Free);
        }
        self.robot_pos = (self.robot_pos.0 - 1, self.robot_pos.1);
        for p in seen.arr.iter_mut() {
            *p = false;
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
enum Space {
    Wall,
    Free,
    Box,
    Robot,
    BoxLeft,
    BoxRight,
}

fn get_input(filename: &str) -> (Map, Vec<Direction>) {
    let contents = fs::read_to_string("input/2024/15/".to_owned() + filename).unwrap();
    let mut parts = contents.split("\n\n");
    let map_str = parts.next().unwrap();
    let dir_str = parts.next().unwrap();

    let mut map_input = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in map_str.split("\n") {
        height += 1;
        width = line.len();
        map_input.extend(line.trim().chars().map(|c| match c {
            'O' => Space::Box,
            '#' => Space::Wall,
            '.' => Space::Free,
            '@' => Space::Robot,
            _ => unreachable!(),
        }));
    }
    let map = Map::new(map_input, width, height);

    let directions = dir_str
        .chars()
        .flat_map(|c| {
            if c.is_whitespace() {
                None
            } else {
                Some(Direction::from_char(c))
            }
        })
        .collect();

    (map, directions)
}

fn process_map(map: &mut Map, directions: &[Direction]) {
    for dir in directions {
        match dir {
            Direction::Up => map.up(),
            Direction::Right => map.right(),
            Direction::Down => map.down(),
            Direction::Left => map.left(),
        }
    }
}

// 1) build up graph of connected spaces (all boxes)
// 2) if any touch a wall, do nothing,
// 3) else move them, via DFS
// 4) update robot position

fn process_map_wide(map: &mut Map, directions: &[Direction]) {
    let mut seen = TwoDArray::new(
        vec![false; map.map.arr.len()],
        map.map.width,
        map.map.height,
    );
    for dir in directions {
        match dir {
            Direction::Up => map.wide_up(&mut seen),
            Direction::Right => map.wide_right(&mut seen),
            Direction::Down => map.wide_down(&mut seen),
            Direction::Left => map.wide_left(&mut seen),
        }
    }
}

fn display_map(map: &Map) {
    for h in 0..map.map.height {
        let tmp_vec: Vec<char> = map.map.arr[h * map.map.width..(h + 1) * map.map.width]
            .iter()
            .map(|c| match c {
                Space::Wall => '#',
                Space::Box => 'O',
                Space::Free => '.',
                Space::Robot => '@',
                Space::BoxLeft => '[',
                Space::BoxRight => ']',
            })
            .collect();
        println!("{:?}", tmp_vec);
    }
    println!();
}

fn sum_gps_of_boxes_after_processing(filename: &str) -> usize {
    let (mut map, directions) = get_input(filename);
    process_map(&mut map, &directions);
    map.map
        .arr
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            if *c == Space::Box {
                (100 * (idx / map.map.width)) + (idx % map.map.width)
            } else {
                0
            }
        })
        .sum()
}

fn widen_map(map: &mut Map) {
    map.map.arr = map
        .map
        .arr
        .iter()
        .flat_map(|c| match c {
            Space::Box => [Space::BoxLeft, Space::BoxRight],
            Space::Wall => [Space::Wall, Space::Wall],
            Space::Free => [Space::Free, Space::Free],
            Space::Robot => [Space::Robot, Space::Free],
            _ => unreachable!(),
        })
        .collect();
    map.map.width *= 2;
    map.robot_pos = (map.robot_pos.0 * 2, map.robot_pos.1);
}

fn wider_sum_gps_of_boxes_after_processing(filename: &str) -> usize {
    let (mut map, directions) = get_input(filename);
    widen_map(&mut map);
    process_map_wide(&mut map, &directions);
    map.map
        .arr
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            if *c == Space::BoxLeft {
                (100 * (idx / map.map.width)) + (idx % map.map.width)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{sum_gps_of_boxes_after_processing, wider_sum_gps_of_boxes_after_processing};

    #[test]
    fn part1_small_example() {
        let result = sum_gps_of_boxes_after_processing("small_example.txt");
        assert_eq!(result, 2028);
    }

    #[test]
    fn part1_large_example() {
        let result = sum_gps_of_boxes_after_processing("large_example.txt");
        assert_eq!(result, 10092);
    }

    #[test]
    fn part1() {
        let result = sum_gps_of_boxes_after_processing("input.txt");
        assert_eq!(result, 1476771);
    }

    #[test]
    fn part2_large_example() {
        let result = wider_sum_gps_of_boxes_after_processing("large_example.txt");
        assert_eq!(result, 9021);
    }

    #[test]
    fn part2() {
        let result = wider_sum_gps_of_boxes_after_processing("input.txt");
        assert_eq!(result, 1468005);
    }
}
