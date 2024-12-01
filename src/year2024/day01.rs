//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use std::collections::HashMap;
use itertools::Itertools;
use adventofcode_rust::aoc::*;

pub fn day01(input: &str) -> SolutionResult {
    let mut list1 = input.lines().map(|line| { line.split_ascii_whitespace().next().unwrap().parse::<i32>().unwrap() } ).collect_vec();
    let mut list2 = input.lines().map(|line| { line.split_ascii_whitespace().last().unwrap().parse::<i32>().unwrap() } ).collect_vec();
    list1.sort_unstable();
    list2.sort_unstable();

    let zipped = list1.iter().zip(list2.iter());
    let a: i32 = zipped.map(|(a, b)| (a - b).abs()).sum();

    let mut list2counts = HashMap::new();
    for i in list2 { *list2counts.entry(i).or_insert(0) += 1; }

    let b: i32 = list1.iter().map(|i| i * *list2counts.entry(*i).or_insert(0)).sum();

    SolutionResult { a: a.to_string(), b: b.to_string()}
}
