//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use std::mem::swap;
use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};
use itertools::Itertools;

pub fn blink(stones: &Vec<i64>, stones_out: &mut Vec<i64>) {
    for stone in stones {
        if *stone == 0 {
            stones_out.push(1);
        } else {
            let digits = (i64::ilog10(*stone)) + 1;
            if digits % 2 == 0 {
                stones_out.push(stone / 10i64.pow(digits / 2));
                stones_out.push(stone % 10i64.pow(digits / 2));
            } else {
                stones_out.push(stone * 2024);
            }
        }
    }
}

pub fn day11(input: &str) -> SolutionResult {
    let mut numbers = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect_vec();
    let mut next = Vec::<i64>::new();

    // println!("{:#?}", numbers);
    for i in 0..25 {
        blink(&numbers, &mut next);
        swap(&mut numbers, &mut next);
        next.clear();
        // println!("{i}: {}: {:#?}", numbers.len(), &numbers.as_slice()[..10]);
    }
    
    let a = numbers.len();

    SolutionResult::new(a, "b")
}
