//
// Copyright (c) 2025 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;
use itertools::Itertools;

pub fn day03(input: &str) -> SolutionResult {
    let lines = input.lines();

    let a = sum_joltage(lines.clone().map(|line| line.as_bytes()), 2);
    let b = sum_joltage(lines.clone().map(|line| line.as_bytes()), 12);

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}

type Joltage = u64;

fn digit_to_joltage(digit: u8) -> Joltage {
    (digit - b'0') as Joltage
}

fn find_digit_distance(line: &[u8]) -> usize {
    line.len() - line.iter().rev().position_max().unwrap() - 1
}

fn find_joltage(line: &[u8], digit_count: usize) -> Joltage {
    assert!(digit_count > 0);
    if digit_count == 1 {
        return digit_to_joltage(line[find_digit_distance(line)]);
    }

    let first_digit_distance = find_digit_distance(&line[0..(line.len() - digit_count + 1)]);

    digit_to_joltage(line[first_digit_distance]) * 10u64.pow((digit_count - 1) as u32)
        + find_joltage(&line[first_digit_distance + 1..], digit_count - 1)
}

fn sum_joltage<'a, I>(lines: I, digit_count: usize) -> Joltage
where
    I: Iterator<Item = &'a [u8]>,
{
    lines
        .map(|line: &[u8]| find_joltage(line, digit_count))
        .sum()
}
