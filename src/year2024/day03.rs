//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::SolutionResult;
use regex::Regex;

pub fn day03(input: &str) -> SolutionResult {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|(do)\(\)|(don't)\(\)").unwrap();

    let mut a = 0i32;
    let mut b = 0i32;
    let mut enabled = true;
    for c in regex.captures_iter(input) {
        if c.get(3).is_some() {
            enabled = true
        } else if c.get(4).is_some() {
            enabled = false
        } else {
            let x: i32 = c.get(1).unwrap().as_str().parse().unwrap();
            let y: i32 = c.get(2).unwrap().as_str().parse().unwrap();
            a += x * y;
            if enabled {
                b += x * y;
            }
        }
    }

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}
