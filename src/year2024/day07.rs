//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::SolutionResult;
use itertools::Itertools;
use rayon::prelude::*;

pub fn day07(input: &str) -> SolutionResult {
    let equations = input
        .lines()
        .map(|line| {
            line.split(|c: char| !c.is_ascii_digit())
                .flat_map(|num| num.parse::<i64>())
        })
        .map(|mut nums| (nums.next().unwrap(), nums.collect_vec()))
        .collect_vec();

    let operators_a = ['+', '*'];
    let operators_b = ['+', '*', '|'];

    let a = solve(&equations, &operators_a);
    let b = solve(&equations, &operators_b);

    SolutionResult::new(a, b)
}

fn solve(equations: &Vec<(i64, Vec<i64>)>, operators: &[char]) -> i64 {
    let longest_equation = equations.iter().map(|(_, eq)| eq.len()).max().unwrap();
    println!("{longest_equation}");
    let mut all_op_sequences: Vec<Vec<Vec<char>>> = Vec::new();
    all_op_sequences.push(vec![]); // op sequence length of 0 is empty
    for op_seq_len in 1..longest_equation {
        let mut op_sequences: Vec<Vec<char>> = operators.iter().map(|op| vec![*op]).collect();
        for _ in 0..(op_seq_len - 1) {
            op_sequences = op_sequences
                .iter()
                .cartesian_product(operators.iter())
                .map(|(sequence, op)| {
                    let mut ret = sequence.clone();
                    ret.push(*op);
                    ret
                })
                .collect_vec();
        }
        all_op_sequences.push(op_sequences);
    }

    equations
        .par_iter()
        .map(|(result, nums)| {
            let op_count = nums.len() - 1;
            for op_seq in &all_op_sequences[op_count - 1] {
                let mut num_iter = nums.iter();
                let mut acc = *num_iter.next().unwrap();
                for op in op_seq {
                    match op {
                        '+' => acc += num_iter.next().unwrap(),
                        '*' => acc *= num_iter.next().unwrap(),
                        '|' => {
                            acc = format!("{}{}", acc, num_iter.next().unwrap())
                                .parse::<i64>()
                                .unwrap()
                        }
                        _ => panic!(),
                    }
                }
                if acc > *result {
                    return 0;
                }
                if acc == *result {
                    return *result;
                }
            }
            0
        })
        .sum()
}
