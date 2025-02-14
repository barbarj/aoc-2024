#![allow(dead_code)]

fn find_lowest_hash_with_five_zeroes(prefix: &str) -> Option<usize> {
    for i in 0usize.. {
        let input = format!("{prefix}{i}");
        let digest = md5::compute(&input);
        let digest_str = format!("{:x}", digest);
        if digest_str.starts_with("00000") {
            println!("{digest_str}");
            return Some(i);
        }
    }
    None
}

fn find_lowest_hash_with_six_zeroes(prefix: &str) -> Option<usize> {
    for i in 0usize.. {
        let input = format!("{prefix}{i}");
        let digest = md5::compute(&input);
        let digest_str = format!("{:x}", digest);
        if digest_str.starts_with("000000") {
            println!("{digest_str}");
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{find_lowest_hash_with_five_zeroes, find_lowest_hash_with_six_zeroes};

    #[test]
    fn part1_example() {
        let input = "abcdef";
        let result = find_lowest_hash_with_five_zeroes(input).unwrap();
        assert_eq!(609043, result);
    }

    #[test]
    fn part1() {
        let input = "iwrupvqb";
        let result = find_lowest_hash_with_five_zeroes(input).unwrap();
        assert_eq!(346386, result);
    }

    #[test]
    fn part2() {
        let input = "iwrupvqb";
        let result = find_lowest_hash_with_six_zeroes(input).unwrap();
        assert_eq!(9958218, result);
    }
}
