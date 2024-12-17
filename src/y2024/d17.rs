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
    fn with_program(program: &[u8]) -> Self {
        Computer {
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            program: program.to_vec(),
            ptr: 0,
        }
    }

    fn combo(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand.into(),
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        }
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

fn three_bits_backtrack(program: &[u8], prefix: usize, len: usize) -> Option<usize> {
    let idx = program.len() - len;
    let mut computer = Computer::with_program(program);
    for i in 0..8 {
        let a = (prefix << 3) + i;
        computer.reg_a = a;
        computer.ptr = 0;
        if computer.run() == program[idx..] {
            if len == program.len() {
                return Some(a);
            } else if let Some(res) = three_bits_backtrack(program, a, len + 1) {
                return Some(res);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{computer_output, load_computer, three_bits_backtrack};

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
        let computer = load_computer("example_part2.txt");
        let result = three_bits_backtrack(&computer.program, 0, 1).unwrap();
        assert_eq!(result, 117440);
    }

    #[test]
    fn part2() {
        let computer = load_computer("input.txt");
        let result = three_bits_backtrack(&computer.program, 0, 1).unwrap();
        assert_eq!(result, 164542125272765);
    }
}
