use std::{collections::HashMap, fs::File, io::Read};

fn parse_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut file = File::open("input/2024/1/".to_owned() + filename).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let nums = buffer.lines().map(|line| {
        let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        (nums.next().unwrap(), nums.next().unwrap())
    });
    nums.collect()
}

#[allow(dead_code)]
fn list_differences(filename: &str) -> u32 {
    let (mut left, mut right) = parse_input(filename);

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

#[allow(dead_code)]
fn similarity_score(filename: &str) -> i32 {
    let (left, right) = parse_input(filename);
    let right: HashMap<i32, i32> = right.iter().fold(HashMap::new(), |mut counts, num| {
        counts.entry(*num).and_modify(|x| *x += 1).or_insert(1);
        counts
    });

    left.iter().map(|x| x * right.get(x).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::{list_differences, similarity_score};

    #[test]
    fn part1_example() {
        let result = list_differences("example.txt");
        assert_eq!(result, 11);
    }

    #[test]
    fn part1() {
        let result = list_differences("input.txt");
        assert_eq!(result, 1590491);
    }

    #[test]
    fn part2_example() {
        let result = similarity_score("example.txt");
        assert_eq!(result, 31);
    }

    #[test]
    fn part2() {
        let result = similarity_score("input.txt");
        assert_eq!(result, 22588371);
    }
}
