//
// Copyright (c) 2022 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;

use itertools::Itertools;

fn priority(c: u8) -> i32 {
    if c as char >= 'a' && c as char <= 'z' {
        return c as i32 - 'a' as i32 + 1
    }
    c as i32 - 'A' as i32 + 27
}

fn priority_mask(items: &[u8]) -> u64 {
    let mut out: u64 = 0;
    for c in items {
        out |= 1 << priority(*c)
    }
    out
}

fn common_items(rucksack: &[u8]) -> u64 {
    let (c1, c2) = rucksack.split_at(rucksack.len()/2);
    priority_mask(c1) & priority_mask(c2)
}

fn common_items_group<I>(rucksack: I) -> u64 where I: Iterator<Item=u64> {
    rucksack.reduce(|acc, x| acc & x).unwrap()
}

fn priority_from_mask(mask: u64) -> i32 {
    mask.trailing_zeros() as i32
}

fn priority_from_rucksack(rucksack: &[u8]) -> i32 {
    priority_from_mask(common_items(rucksack))
}

pub fn day03(input: &str) -> SolutionResult {
    let rucksacks =
        input.trim()
            .split("\n")
            .map(|l| l.as_bytes())
            .collect::<Vec<&[u8]>>();

    let part1_sum: i32 = rucksacks.iter()
        .map(|x| priority_from_rucksack(x))
        .sum();

    let part2_sum: i32 = rucksacks.iter()
        .map(|x| priority_mask(x))
        .chunks(3).into_iter()
        .map(common_items_group)
        .map(priority_from_mask)
        .sum();

    SolutionResult {
        a: part1_sum.to_string(),
        b: part2_sum.to_string(),
    }
}
