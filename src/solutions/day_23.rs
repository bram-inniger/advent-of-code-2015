use std::str::FromStr;

use rustc_hash::FxHashMap;

pub fn solve_1(instructions: &[&str]) -> u32 {
    let mut computer = Computer::new(instructions);
    computer.run();
    computer.registers[&Register::B]
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Computer {
    instructions: Vec<Instruction>,
    registers: FxHashMap<Register, u32>,
    ip: usize,
}

impl Computer {
    fn new(instructions: &[&str]) -> Self {
        Self {
            instructions: instructions.iter().map(|&i| Instruction::new(i)).collect(),
            registers: [Register::A, Register::B]
                .into_iter()
                .map(|r| (r, 0))
                .collect(),
            ip: 0,
        }
    }

    fn run(&mut self) {
        loop {
            if self.ip >= self.instructions.len() {
                return;
            }

            let instruction = &self.instructions[self.ip];
            match instruction {
                Instruction::Half { r } => {
                    *self.registers.get_mut(r).unwrap() /= 2;
                    self.ip += 1;
                }
                Instruction::Triple { r } => {
                    *self.registers.get_mut(r).unwrap() *= 3;
                    self.ip += 1;
                }
                Instruction::Inc { r } => {
                    *self.registers.get_mut(r).unwrap() += 1;
                    self.ip += 1;
                }
                Instruction::Jump { offset } => self.ip = (self.ip as i32 + offset) as usize,
                Instruction::JumpIfEven { r, offset } => {
                    if *self.registers.get_mut(r).unwrap() % 2 == 0 {
                        self.ip = (self.ip as i32 + offset) as usize
                    } else {
                        self.ip += 1;
                    }
                }
                Instruction::JumpIfOne { r, offset } => {
                    if *self.registers.get_mut(r).unwrap() == 1 {
                        self.ip = (self.ip as i32 + offset) as usize
                    } else {
                        self.ip += 1;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Instruction {
    Half { r: Register },
    Triple { r: Register },
    Inc { r: Register },
    Jump { offset: i32 },
    JumpIfEven { r: Register, offset: i32 },
    JumpIfOne { r: Register, offset: i32 },
}

impl Instruction {
    fn new(instruction: &str) -> Self {
        match &instruction[0..3] {
            "hlf" => Instruction::Half {
                r: Register::new(&instruction[4..]),
            },
            "tpl" => Instruction::Triple {
                r: Register::new(&instruction[4..]),
            },
            "inc" => Instruction::Inc {
                r: Register::new(&instruction[4..]),
            },
            "jmp" => Instruction::Jump {
                offset: Self::offset(&instruction[4..]),
            },
            "jie" => Instruction::JumpIfEven {
                r: Register::new(&instruction[4..5]),
                offset: Self::offset(&instruction[7..]),
            },
            "jio" => Instruction::JumpIfOne {
                r: Register::new(&instruction[4..5]),
                offset: Self::offset(&instruction[7..]),
            },
            _ => unreachable!(),
        }
    }

    fn offset(offset: &str) -> i32 {
        i32::from_str(offset).unwrap()
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Register {
    A,
    B,
}

impl Register {
    fn new(register: &str) -> Self {
        match register {
            "a" => Register::A,
            "b" => Register::B,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_23_part_01_sample() {
        let sample = vec!["inc a", "jio a, +2", "tpl a", "inc a"];

        assert_eq!(0, solve_1(&sample));
    }

    #[test]
    fn day_23_part_01_solution() {
        let input = include_str!("../../inputs/day_23.txt")
            .lines()
            .collect_vec();

        assert_eq!(255, solve_1(&input));
    }
}
