#![allow(dead_code)]

use std::{collections::HashMap, fs::File, io::Read};

fn load_input(filename: &str) -> Vec<usize> {
    let mut file = File::open("input/11/".to_owned() + filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn digit_count(mut v: usize) -> usize {
    let mut count = 1;
    while v >= 10 {
        v /= 10;
        count += 1;
    }
    count
}

// memoized key is (v, steps)
fn count_after_applying_rules(
    v: usize,
    steps: usize,
    memoized: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if steps == 0 {
        return 1;
    }
    if let Some(count) = memoized.get(&(v, steps)) {
        return *count;
    }
    let count = if v == 0 {
        count_after_applying_rules(1, steps - 1, memoized)
    } else if digit_count(v) % 2 == 0 {
        let divisor = 10usize.pow((digit_count(v) / 2).try_into().unwrap());
        count_after_applying_rules(v / divisor, steps - 1, memoized)
            + count_after_applying_rules(v % divisor, steps - 1, memoized)
    } else {
        count_after_applying_rules(v * 2024, steps - 1, memoized)
    };
    memoized.insert((v, steps), count);
    count
}

fn count_nums_after_steps(input: Vec<usize>, steps: usize) -> usize {
    let mut memoized = HashMap::new();
    input
        .into_iter()
        .map(|v| count_after_applying_rules(v, steps, &mut memoized))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{count_nums_after_steps, digit_count, load_input};

    fn apply_rules_to_vec(input: Vec<usize>, steps: usize) -> Vec<usize> {
        let mut new_vec = input;
        for _ in 0..steps {
            println!("{new_vec:?}");
            let last_vec = new_vec;
            new_vec = Vec::new();
            for num in last_vec {
                if num == 0 {
                    new_vec.push(1);
                } else if digit_count(num) % 2 == 0 {
                    let divisor = 10usize.pow((digit_count(num) / 2).try_into().unwrap());
                    new_vec.push(num / divisor);
                    new_vec.push(num % divisor);
                } else {
                    new_vec.push(num * 2024);
                }
            }
        }
        println!("{new_vec:?}");
        new_vec
    }

    #[test]
    fn part1_example() {
        let input = load_input("example.txt");
        let result = count_nums_after_steps(input, 25);
        assert_eq!(result, 55312);
    }

    #[test]
    fn part1() {
        let input = load_input("input.txt");
        let result = count_nums_after_steps(input, 25);
        assert_eq!(result, 209412);
    }

    #[test]
    fn part2() {
        let input = load_input("input.txt");
        let result = count_nums_after_steps(input, 75);
        assert_eq!(result, 248967696501656);
    }

    #[test]
    fn cycle_testing() {
        let input = vec![1];
        let _result = apply_rules_to_vec(input, 10);
    }
}
