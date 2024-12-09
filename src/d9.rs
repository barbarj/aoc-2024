#![allow(dead_code)]
use std::{collections::VecDeque, fs};

fn load_input(filename: &str) -> Vec<u8> {
    let contents = fs::read_to_string("input/9/".to_owned() + filename).unwrap();
    contents.trim().bytes().map(|b| b - b'0').collect()
}

fn checksum_after_moving_blocks(filename: &str) -> usize {
    let mut disk = load_input(filename);
    assert!((disk.len() - 1) % 2 == 0);
    let mut left = 0;
    let mut block_position: usize = 0;
    let mut right = disk.len() - 1;
    let mut checksum: usize = 0;
    while left < right {
        while left < right && disk[left] > 0 {
            let file_id = if left % 2 == 0 { left / 2 } else { right / 2 };
            checksum += block_position * file_id;
            block_position += 1;
            if left % 2 == 1 {
                disk[right] -= 1;
                if disk[right] == 0 {
                    right -= 2;
                }
            }
            disk[left] -= 1;
        }
        left += 1;
    }
    while left == right && disk[left] > 0 {
        let file_id = left / 2;
        checksum += block_position * file_id;
        block_position += 1;
        disk[left] -= 1;
    }
    checksum
}

fn checksum_after_moving_files(filename: &str) -> usize {
    let disk = load_input(filename);
    let mut block_position = 0;
    let mut files = Vec::with_capacity(disk.len() / 2);
    let mut free_space = VecDeque::with_capacity(disk.len() / 2);
    for (idx, space) in disk.iter().enumerate() {
        let space = *space as usize;
        if idx % 2 == 0 {
            files.push((idx / 2, block_position..block_position + space));
        } else {
            free_space.push_back(block_position..block_position + space);
        }
        block_position += space;
    }
    let mut placed = Vec::with_capacity(disk.len() / 2);

    while let Some(file) = files.pop() {
        if let Some(move_to_idx) = free_space
            .iter()
            .enumerate()
            .find(|(_, range)| range.len() >= file.1.len())
            .map(|(idx, _)| idx)
        {
            let move_to = free_space.get_mut(move_to_idx).unwrap();
            placed.push((file.0, (move_to.start..move_to.start + file.1.len())));
            move_to.start += file.1.len();
            while free_space.front().filter(|x| x.is_empty()).is_some() {
                let _ = free_space.pop_front();
            }
        } else {
            placed.push(file);
        }
        let _ = free_space.pop_back();
    }

    placed
        .into_iter()
        .map(|(file_id, range)| range.map(|x| x * file_id).sum::<usize>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{checksum_after_moving_blocks, checksum_after_moving_files};

    #[test]
    fn part1_example() {
        let result = checksum_after_moving_blocks("example.txt");
        assert_eq!(result, 1928);
    }

    #[test]
    fn part1() {
        let result = checksum_after_moving_blocks("input.txt");
        assert_eq!(result, 6154342787400);
    }

    #[test]
    fn part2_example() {
        let result = checksum_after_moving_files("example.txt");
        assert_eq!(result, 2858);
    }

    #[test]
    fn part2() {
        let result = checksum_after_moving_files("input.txt");
        assert_eq!(result, 6183632723350);
    }
}
