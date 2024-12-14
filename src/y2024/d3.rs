use std::{fs::File, io::Read};

use regex::Regex;

#[allow(dead_code)]
fn get_input(filename: &str) -> String {
    let mut file = File::open("input/2024/3/".to_owned() + filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[allow(dead_code)]
fn add_mutliply_instructions(filename: &str) -> u32 {
    let input = get_input(filename);
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&input)
        .map(|c| {
            let captures: (&str, [&str; 2]) = c.extract();
            let a = captures.1[0].parse::<u32>().unwrap();
            let b = captures.1[1].parse::<u32>().unwrap();
            a * b
        })
        .sum()
}

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    let all_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();
    let multiply_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    all_re
        .find_iter(input)
        .map(|m| match m.as_str() {
            "don't()" => Instruction::Dont,
            "do()" => Instruction::Do,
            mul => {
                let captures: [&str; 2] = multiply_re.captures(mul).unwrap().extract().1;
                let a = captures[0].parse::<u32>().unwrap();
                let b = captures[1].parse::<u32>().unwrap();
                Instruction::Mul(a, b)
            }
        })
        .collect()
}

#[allow(dead_code)]
fn sum_multiplies_with_instructions(filename: &str) -> u32 {
    let input = get_input(filename);
    let instructions = get_instructions(&input);
    let mut do_state = true;
    let mut multiplies = Vec::new();
    for ins in instructions {
        match (ins, do_state) {
            (Instruction::Do, _) => do_state = true,
            (Instruction::Dont, _) => do_state = false,
            (Instruction::Mul(a, b), true) => multiplies.push(a * b),
            (Instruction::Mul(_, _), false) => (),
        }
    }
    multiplies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::{add_mutliply_instructions, sum_multiplies_with_instructions};

    #[test]
    fn part1_example() {
        let result = add_mutliply_instructions("example.txt");
        assert_eq!(result, 161);
    }

    #[test]
    fn part1() {
        let result = add_mutliply_instructions("input.txt");
        assert_eq!(result, 181345830);
    }

    #[test]
    fn part2_example() {
        let result = sum_multiplies_with_instructions("example2.txt");
        assert_eq!(result, 48);
    }

    #[test]
    fn part2() {
        let result = sum_multiplies_with_instructions("input.txt");
        assert_eq!(result, 98729041);
    }
}
