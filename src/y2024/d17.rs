#![allow(dead_code)]

use std::fs;

use itertools::Itertools;

struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<u8>,
    ptr: usize,
}
impl Computer {
    fn combo(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand.into(),
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        }
    }

    fn run_to_match(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        while self.ptr < self.program.len() {
            if let Some(out) = self.step() {
                if out != self.program[output.len()] {
                    return output;
                }
                output.push(out);
            }
        }
        output
    }

    fn run(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        while self.ptr < self.program.len() {
            if let Some(out) = self.step() {
                output.push(out);
            }
        }
        output
    }

    fn step(&mut self) -> Option<u8> {
        let opcode = self.program[self.ptr];
        let operand = self.program[self.ptr + 1];
        let mut out = None;
        match opcode {
            0 => {
                self.reg_a /= 2usize.pow(self.combo(operand).try_into().unwrap());
                self.ptr += 2;
            }
            1 => {
                self.reg_b ^= operand as usize;
                self.ptr += 2;
            }
            2 => {
                self.reg_b = self.combo(operand) % 8;
                self.ptr += 2;
            }
            3 => {
                if self.reg_a != 0 {
                    self.ptr = operand as usize;
                } else {
                    self.ptr += 2;
                }
            }
            4 => {
                self.reg_b ^= self.reg_c;
                self.ptr += 2;
            }
            5 => {
                out = Some((self.combo(operand) % 8).try_into().unwrap());
                self.ptr += 2;
            }
            6 => {
                self.reg_b = self.reg_a / 2usize.pow(self.combo(operand).try_into().unwrap());
                self.ptr += 2;
            }
            7 => {
                self.reg_c = self.reg_a / 2usize.pow(self.combo(operand).try_into().unwrap());
                self.ptr += 2;
            }
            _ => unreachable!(),
        }

        out
    }
}

fn load_computer(filename: &str) -> Computer {
    let contents = fs::read_to_string("input/2024/17/".to_owned() + filename).unwrap();
    let mut lines = contents.lines();
    let reg_a = lines.next().unwrap()[12..].parse::<usize>().unwrap();
    let reg_b = lines.next().unwrap()[12..].parse::<usize>().unwrap();
    let reg_c = lines.next().unwrap()[12..].parse::<usize>().unwrap();

    lines.next();
    let program_str = &lines.next().unwrap()[9..];
    let program = program_str
        .trim()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    Computer {
        reg_a,
        reg_b,
        reg_c,
        program,
        ptr: 0,
    }
}

fn computer_output(filename: &str) -> String {
    let mut computer = load_computer(filename);
    let output = computer.run();
    output.iter().join(",")
}

fn build_in_reverse() -> usize {
    let program = [2u8, 4, 1, 1, 7, 5, 1, 5, 0, 3, 4, 3, 5, 5, 3, 0];
    let mut a = 0;
    for (idx, num) in program.iter().enumerate().rev() {
        let mut i = 0;
        while i < 8 {
            let tmp_a = a + i;
            let mut b = (tmp_a % 8) ^ 1;
            let c = tmp_a / 2usize.pow(b.try_into().unwrap());
            b = (b ^ 5) ^ c;
            println!("{i} -> {}, {}", tmp_a, b % 8);
            if b % 8 == *num as usize {
                break;
            }
            i += 1;
        }
        if i == 8 {
            println!("broke at idx {idx}");
            break;
        }
        a = (a + i) * 8;
    }
    a / 8
}

pub fn correct_val_to_make_quine(filename: &str) -> usize {
    let mut computer = load_computer(filename);
    let mut i = 0;
    loop {
        if i % 1000 == 0 {
            println!("{i}");
        }
        computer.reg_a = i;
        computer.reg_b = 0;
        computer.reg_c = 0;
        computer.ptr = 0;
        let output = computer.run_to_match();
        if output == computer.program {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::load_computer;
    use super::{computer_output, correct_val_to_make_quine};

    #[test]
    fn part1_example() {
        let result = computer_output("example.txt");
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part1() {
        let result = computer_output("input.txt");
        assert_eq!(result, "4,1,5,3,1,5,3,5,7");
    }

    #[test]
    fn part2_example() {
        let result = correct_val_to_make_quine("example_part2.txt");
        assert_eq!(result, 117440);
    }

    #[test]
    fn part2_real() {
        let mut computer = load_computer("input.txt");
        let mut a = 20567765534425 * 8;
        loop {
            // 20567765534425 is the reg_a value that produces the entire program except the first
            //                operator. Figured that out using build_in_reverse
            a += 1;
            computer.reg_a = a;
            computer.ptr = 0;
            let res = computer.run();
            if res == computer.program {
                break;
            }
        }
        assert_eq!(a, 164542125272765);
    }
}
