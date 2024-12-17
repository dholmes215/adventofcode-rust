//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::SolutionResult;
use itertools::Itertools;

pub fn day17(input: &str) -> SolutionResult {
    let mut computer = parse_input(input);
    computer.run();
    let a = computer.output.iter().map(i64::to_string).join(",");

    let reverse_program = computer.program.into_iter().rev().collect_vec();
    let mut a_register_candidates: Vec<i64> = vec![0];
    for b_after in reverse_program {
        let mut next_candidates = vec![];
        for a_after in &a_register_candidates {
            next_candidates.extend(reverse_step(*a_after, b_after));
        }
        a_register_candidates = next_candidates;
    }
    let b = a_register_candidates.iter().min().unwrap();

    SolutionResult::new(a, b)
}

// Given the values of a and b after the step, return the
// possible values that a could have had before the step.
fn reverse_step(a_after: i64, b_after: i64) -> Vec<i64> {
    let mut out = vec![];
    for a_before in (a_after * 8)..(a_after + 1) * 8 {
        let candidate = davids_program_step(a_before);
        if candidate == b_after {
            out.push(a_before);
        }
    }
    out
}

#[derive(Clone, Debug)]
struct Computer {
    registers: [i64; 3],
    instruction_ptr: usize,
    program: Vec<i64>,
    output: Vec<i64>,
}

impl Computer {
    fn a(&mut self) -> &mut i64 {
        &mut self.registers[0]
    }

    fn b(&mut self) -> &mut i64 {
        &mut self.registers[1]
    }

    fn c(&mut self) -> &mut i64 {
        &mut self.registers[2]
    }

    fn run(&mut self) {
        while self.instruction_ptr < self.program.len() {
            self.run_instruction();
        }
    }

    fn combo(&self, opcode: i64) -> i64 {
        match opcode {
            0..=3 => opcode,
            4..=6 => self.registers[opcode as usize - 4],
            7 => panic!(), // Reserved
            _ => panic!(),
        }
    }

    fn run_instruction(&mut self) {
        let instruction = self.program[self.instruction_ptr];
        let opcode = self.program[self.instruction_ptr + 1];
        match instruction {
            0 => {
                // adv
                let numerator = *self.a();
                let denominator = 2i64.pow(self.combo(opcode) as u32);
                *self.a() = numerator / denominator;
                self.instruction_ptr += 2;
            }
            1 => {
                // bxl
                *self.b() ^= opcode;
                self.instruction_ptr += 2;
            }
            2 => {
                // bst
                *self.b() = self.combo(opcode) % 8;
                self.instruction_ptr += 2;
            }
            3 => {
                // jnz
                if *self.a() == 0 {
                    self.instruction_ptr += 2;
                } else {
                    self.instruction_ptr = opcode as usize;
                }
            }
            4 => {
                // bxc
                *self.b() ^= *self.c();
                self.instruction_ptr += 2;
            }
            5 => {
                // out
                self.output.push(self.combo(opcode) % 8);
                self.instruction_ptr += 2;
            }
            6 => {
                // bdv
                let numerator = *self.a();
                let denominator = 2i64.pow(self.combo(opcode) as u32);
                *self.b() = numerator / denominator;
                self.instruction_ptr += 2;
            }
            7 => {
                // cdv
                let numerator = *self.a();
                let denominator = 2i64.pow(self.combo(opcode) as u32);
                *self.c() = numerator / denominator;
                self.instruction_ptr += 2;
            }
            _ => panic!(),
        }
    }
}

fn parse_input(input: &str) -> Computer {
    let mut nums = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap());
    Computer {
        registers: [
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
        ],
        instruction_ptr: 0,
        program: nums.collect(),
        output: vec![],
    }
}

// This is one iteration of my own input program translated into Rust
fn davids_program_step(a: i64) -> i64 {
    let b = (a % 8) ^ 3;
    let c = a / 2_i64.pow(b as u32);
    (b ^ 5 ^ c) % 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut c = Computer {
            registers: [0, 0, 9],
            instruction_ptr: 0,
            program: vec![2, 6],
            output: vec![],
        };
        c.run();
        assert_eq!(*c.b(), 1);
    }

    #[test]
    fn test2() {
        let mut c = Computer {
            registers: [10, 0, 0],
            instruction_ptr: 0,
            program: vec![5, 0, 5, 1, 5, 4],
            output: vec![],
        };
        c.run();
        assert_eq!(c.output, vec![0, 1, 2]);
    }

    #[test]
    fn test3() {
        let mut c = Computer {
            registers: [2024, 0, 0],
            instruction_ptr: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            output: vec![],
        };
        c.run();
        assert_eq!(c.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(*c.a(), 0)
    }

    #[test]
    fn test4() {
        let mut c = Computer {
            registers: [0, 29, 0],
            instruction_ptr: 0,
            program: vec![1, 7],
            output: vec![],
        };
        c.run();
        assert_eq!(*c.b(), 26)
    }

    #[test]
    fn test5() {
        let mut c = Computer {
            registers: [0, 2024, 43690],
            instruction_ptr: 0,
            program: vec![4, 0],
            output: vec![],
        };
        c.run();
        assert_eq!(*c.b(), 44354)
    }
}
