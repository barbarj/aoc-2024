#![allow(dead_code)]

use std::fs;

use regex::Regex;

#[derive(Debug)]
struct RectPrism {
    length: i16,
    width: i16,
    height: i16,
}
impl RectPrism {
    fn new(length: i16, width: i16, height: i16) -> Self {
        RectPrism {
            length,
            width,
            height,
        }
    }

    fn volume(&self) -> i32 {
        self.length as i32 * self.width as i32 * self.height as i32
    }

    fn ribbon_needed(&self) -> i32 {
        let l: i32 = self.length.into();
        let w: i32 = self.width.into();
        let h: i32 = self.height.into();

        let perimeters = [2 * (l + w), 2 * (l + h), 2 * (w + h)];
        perimeters.iter().min().unwrap() + self.volume()
    }

    fn paper_needed(&self) -> i32 {
        let l: i32 = self.length.into();
        let w: i32 = self.width.into();
        let h: i32 = self.height.into();

        let sides = [(l * w), (l * h), (w * h)];
        sides.iter().map(|s| s * 2).sum::<i32>() + sides.iter().min().unwrap()
    }
}

fn get_input(filename: &str) -> Vec<RectPrism> {
    let contents = fs::read_to_string("input/2015/2/".to_owned() + filename).unwrap();
    let pattern = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    contents
        .lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            RectPrism::new(
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            )
        })
        .collect()
}

fn sum_paper_needed(filename: &str) -> i32 {
    let prisms = get_input(filename);
    prisms.iter().map(|p| p.paper_needed()).sum()
}

fn sum_ribbon_needed(filename: &str) -> i32 {
    let prisms = get_input(filename);
    prisms.iter().map(|p| p.ribbon_needed()).sum()
}

#[cfg(test)]
mod tests {
    use super::{sum_paper_needed, sum_ribbon_needed};

    #[test]
    fn part1() {
        let result = sum_paper_needed("input.txt");
        assert_eq!(result, 1586300);
    }

    #[test]
    fn part2() {
        let result = sum_ribbon_needed("input.txt");
        assert_eq!(result, 3737498);
    }
}
