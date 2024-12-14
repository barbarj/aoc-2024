use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(dead_code)]
fn load_input(filename: &str) -> Vec<Vec<u8>> {
    let file = File::open("input/2024/4/".to_owned() + filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().bytes().collect())
        .collect()
}

#[allow(dead_code)]
fn count_xmas_occurrences(filename: &str) -> u32 {
    let lines = load_input(filename);
    let mut count = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if j <= lines[i].len() - 4 {
                // right or left
                if lines[i][j..j + 4] == *"XMAS".as_bytes()
                    || lines[i][j..j + 4] == *"SAMX".as_bytes()
                {
                    count += 1;
                }
                // up-right or down-left
                if i >= 3
                    && ((lines[i][j] == b'X'
                        && lines[i - 1][j + 1] == b'M'
                        && lines[i - 2][j + 2] == b'A'
                        && lines[i - 3][j + 3] == b'S')
                        || (lines[i][j] == b'S'
                            && lines[i - 1][j + 1] == b'A'
                            && lines[i - 2][j + 2] == b'M'
                            && lines[i - 3][j + 3] == b'X'))
                {
                    count += 1;
                }
                // down-right or up-left
                if i <= lines.len() - 4
                    && ((lines[i][j] == b'X'
                        && lines[i + 1][j + 1] == b'M'
                        && lines[i + 2][j + 2] == b'A'
                        && lines[i + 3][j + 3] == b'S')
                        || (lines[i][j] == b'S'
                            && lines[i + 1][j + 1] == b'A'
                            && lines[i + 2][j + 2] == b'M'
                            && lines[i + 3][j + 3] == b'X'))
                {
                    count += 1;
                }
            }
            // down or up
            if i <= lines.len() - 4
                && ((lines[i][j] == b'X'
                    && lines[i + 1][j] == b'M'
                    && lines[i + 2][j] == b'A'
                    && lines[i + 3][j] == b'S')
                    || (lines[i][j] == b'S'
                        && lines[i + 1][j] == b'A'
                        && lines[i + 2][j] == b'M'
                        && lines[i + 3][j] == b'X'))
            {
                count += 1;
            }
        }
    }
    count
}

#[allow(dead_code)]
fn count_x_mas_occurrences(filename: &str) -> u32 {
    let lines = load_input(filename);
    let mut count = 0;
    for i in 1..lines.len() - 1 {
        for j in 1..lines[i].len() - 1 {
            // check A
            if lines[i][j] == b'A'
                // down down
                && ((lines[i - 1][j - 1] == b'M'
                    && lines[i - 1][j + 1] == b'M'
                    && lines[i + 1][j - 1] == b'S'
                    && lines[i + 1][j + 1] == b'S')
                    // down up
                    || (lines[i - 1][j - 1] == b'M'
                        && lines[i - 1][j + 1] == b'S'
                        && lines[i + 1][j - 1] == b'M'
                        && lines[i + 1][j + 1] == b'S')
                    // up down
                    || (lines[i - 1][j - 1] == b'S'
                        && lines[i - 1][j + 1] == b'M'
                        && lines[i + 1][j - 1] == b'S'
                        && lines[i + 1][j + 1] == b'M')
                    // up up
                    || (lines[i - 1][j - 1] == b'S'
                        && lines[i - 1][j + 1] == b'S'
                        && lines[i + 1][j - 1] == b'M'
                        && lines[i + 1][j + 1] == b'M'))
            {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::{count_x_mas_occurrences, count_xmas_occurrences};

    #[test]
    fn part1_example() {
        let result = count_xmas_occurrences("example.txt");
        assert_eq!(result, 18);
    }

    #[test]
    fn part1() {
        let result = count_xmas_occurrences("input.txt");
        assert_eq!(result, 2493);
    }

    #[test]
    fn part2_example() {
        let result = count_x_mas_occurrences("example.txt");
        assert_eq!(result, 9);
    }

    #[test]
    fn part2() {
        let result = count_x_mas_occurrences("input.txt");
        assert_eq!(result, 1890);
    }
}
