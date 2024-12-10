#![allow(dead_code)]
use std::{collections::HashSet, fs::File, io::Read};

fn load_map(filename: &str) -> Vec<Vec<u8>> {
    let mut file = File::open("input/10/".to_owned() + filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .lines()
        .map(|line| line.bytes().map(|c| c - b'0').collect())
        .collect()
}

fn look_for_summits(map: &Vec<Vec<u8>>, reachable: &mut Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    if reachable[i][j] > 0 {
        return reachable[i][j];
    }
    let v = map[i][j];
    if v == 9 {
        return 1;
    }
    let mut count = 0;
    if i > 0 && map[i - 1][j] == v + 1 {
        count += look_for_summits(map, reachable, i - 1, j);
    }
    if j < map[i].len() - 1 && map[i][j + 1] == v + 1 {
        count += look_for_summits(map, reachable, i, j + 1);
    }
    if i < map.len() - 1 && map[i + 1][j] == v + 1 {
        count += look_for_summits(map, reachable, i + 1, j);
    }
    if j > 0 && map[i][j - 1] == v + 1 {
        count += look_for_summits(map, reachable, i, j - 1)
    }
    reachable[i][j] = count;
    count
}

fn count_possible_paths(filename: &str) -> u32 {
    let map = load_map(filename);
    let mut reachable_summits: Vec<Vec<u32>> =
        map.iter().map(|line| vec![0u32; line.len()]).collect();

    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                count += look_for_summits(&map, &mut reachable_summits, i, j);
            }
        }
    }
    count
}

fn set_reachable_summit_coords(
    map: &Vec<Vec<u8>>,
    reached: &mut Vec<(usize, usize)>,
    i: usize,
    j: usize,
) {
    if map[i][j] == 9 {
        reached.push((i, j));
        return;
    }
    let v = map[i][j];
    if i > 0 && map[i - 1][j] == v + 1 {
        set_reachable_summit_coords(map, reached, i - 1, j);
    }
    if j < map[i].len() - 1 && map[i][j + 1] == v + 1 {
        set_reachable_summit_coords(map, reached, i, j + 1);
    }
    if i < map.len() - 1 && map[i + 1][j] == v + 1 {
        set_reachable_summit_coords(map, reached, i + 1, j);
    }
    if j > 0 && map[i][j - 1] == v + 1 {
        set_reachable_summit_coords(map, reached, i, j - 1);
    }
}

fn count_reachable_summits(filename: &str) -> usize {
    let map = load_map(filename);
    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                let mut reached = Vec::new();
                set_reachable_summit_coords(&map, &mut reached, i, j);
                let distinct: HashSet<(usize, usize)> = reached.into_iter().collect();
                count += distinct.len();
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::{count_possible_paths, count_reachable_summits};

    #[test]
    fn part1_example() {
        let result = count_reachable_summits("example.txt");
        assert_eq!(result, 36);
    }

    #[test]
    fn part1() {
        let result = count_reachable_summits("input.txt");
        assert_eq!(result, 574);
    }

    #[test]
    fn part2_example() {
        let result = count_possible_paths("example.txt");
        assert_eq!(result, 81);
    }

    #[test]
    fn part2() {
        let result = count_possible_paths("input.txt");
        assert_eq!(result, 1238);
    }
}
