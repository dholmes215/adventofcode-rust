//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::SolutionResult;
use std::collections::HashMap;

pub fn blink(stones: &HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut stones_out: HashMap<i64, i64> = HashMap::new();
    for (stone, count) in stones {
        if *stone == 0 {
            *stones_out.entry(1).or_insert(0) += count;
        } else {
            let digits = (i64::ilog10(*stone)) + 1;
            if digits % 2 == 0 {
                *stones_out.entry(stone / 10i64.pow(digits / 2)).or_insert(0) += count;
                *stones_out.entry(stone % 10i64.pow(digits / 2)).or_insert(0) += count;
            } else {
                *stones_out.entry(stone * 2024).or_insert(0) += count;
            }
        }
    }

    stones_out
}

pub fn day11(input: &str) -> SolutionResult {
    let mut number_counts: HashMap<i64, i64> = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .for_each(|n| *number_counts.entry(n).or_insert(0) += 1);

    for _ in 0..25 {
        number_counts = blink(&number_counts);
    }
    let a = number_counts.values().sum::<i64>();

    for _ in 0..50 {
        number_counts = blink(&number_counts);
    }
    let b = number_counts.values().sum::<i64>();

    SolutionResult::new(a, b)
}
