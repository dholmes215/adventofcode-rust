//
// Copyright (c) 2022 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;

fn not_unique(window: &[u8]) -> bool {
    let mut mask: u64 = 0;
    for c in window {
        mask |= 1 << (*c - b'A')
    }
    mask.count_ones() as usize != window.len()
}

pub fn day06(input: &str) -> SolutionResult {
    let trimmed_bytes = input.trim().as_bytes();
    let part1 = trimmed_bytes.windows(4).take_while(|x| not_unique(x)).count() + 4;
    let part2 = trimmed_bytes.windows(14).take_while(|x| not_unique(x)).count() + 14;

    SolutionResult {
        a: part1.to_string(),
        b: part2.to_string(),
    }
}
