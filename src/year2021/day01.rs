//
// Copyright (c) 2022 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;

fn parse_input(input: &str) -> Vec<i32> {
    input.split_whitespace()
        .map(|x| x.parse::<i32>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect::<Vec<i32>>()
}

fn count_depth_increases(v: &Vec<i32>, window_size: usize) -> usize {
    // Part B explanation: the problem asks us to compare the sums of the
    // elements of overlapping windows of size 3:
    //   (a + b + c) < (b + c + d)
    // Subtract (b + c) from both sides and this can be simplified to:
    //   a < d
    // Thus instead of comparing two windows of size 3, we can compare the first
    // and last elements of one window of size 4.  Part A involved comparing the
    // first and last elements of one window of size 2, so the two parts are
    // really the same general problem with different window sizes.
    let depth_increased = |w: &&[i32]| w.get(window_size - 1) > w.get(0);
    v.windows(window_size)
        .filter(depth_increased)
        .count()
}

pub fn day01(input: &str) -> SolutionResult {
    let parsed = parse_input(input);
    let result = SolutionResult {
        a: count_depth_increases(&parsed,2).to_string(),
        b: count_depth_increases(&parsed,4).to_string(),
    };
    result
}