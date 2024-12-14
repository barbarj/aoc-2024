#![allow(dead_code)]
use std::fs;

fn load_input(filename: &str) -> Vec<u8> {
    let contents = fs::read_to_string("input/2024/9/".to_owned() + filename).unwrap();
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
    let mut free_space: [Vec<usize>; 10] = [
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
    ];
    for (idx, space) in disk.iter().enumerate() {
        let space = *space as usize;
        if idx % 2 == 0 {
            files.push((idx / 2, block_position..block_position + space));
        } else if space != 0 {
            free_space[space].push(block_position);
        }
        block_position += space;
    }
    for fs in free_space.iter_mut() {
        fs.reverse();
    }

    let mut checksum = 0;
    while let Some(file) = files.pop() {
        let file_size = file.1.len();
        if let Some(i) = free_space
            .iter()
            .enumerate()
            .flat_map(|(idx, spaces)| spaces.last().map(|x| (idx, x)))
            .filter(|(idx, pos)| *idx >= file_size && **pos < file.1.start)
            .min_by_key(|(_, pos)| **pos)
            .map(|(idx, _)| idx)
        {
            let pos = free_space[i].pop().unwrap();
            checksum += (pos..pos + file_size).map(|x| x * file.0).sum::<usize>();
            if i != file_size {
                let new_space_size = i - file_size;
                let new_pos = pos + file_size;
                let insert_idx = free_space[new_space_size]
                    .binary_search_by(|x| x.cmp(&new_pos).reverse())
                    .unwrap_err();
                free_space[new_space_size].insert(insert_idx, new_pos);
            }
        } else {
            checksum += file.1.map(|x| x * file.0).sum::<usize>();
        }
    }
    checksum
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
