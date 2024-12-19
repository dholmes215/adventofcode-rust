// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::SolutionResult;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::{Copied, Filter};
use std::slice::Iter;

pub fn day19(input: &str) -> SolutionResult {
    let mut lines = input.lines();
    let mut towels = lines.next().unwrap().split(", ").collect_vec();
    towels.sort();
    let patterns = lines.skip(1).collect_vec();

    let result = patterns.iter()
        .map(|p| test_pattern(p, &towels)).collect_vec();
    let a = result
        .iter()
        .filter(|i| **i > 0u64)
        .count();
    let b = result
        .iter()
        .sum::<u64>();

    SolutionResult::new(a, b)
}

// Return the subset of the given towels that could be the first towel of the given pattern
fn next_towel_options<'a>(
    pattern: &'a str,
    sorted_towels: &'a [&'a str],
) -> impl Iterator<Item = &'a str> {
    sorted_towels
        .iter()
        .filter(|towel| pattern.starts_with(*towel))
        .copied()
}

fn test_pattern(pattern: &str, towels: &[&str]) -> u64 {
    let mut cache = HashMap::new();
    test_pattern_cached(pattern, towels, &mut cache)
}

fn test_pattern_cached<'a>(
    pattern: &'a str,
    towels: &[&str],
    memos: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(result) = memos.get(pattern) {
        return *result;
    }
    let matching_towels = next_towel_options(pattern, towels);

    let mut count = 0u64;
    for towel in matching_towels {
        if *pattern == *towel {
            count += 1;
        }
        if pattern.starts_with(towel) {
            count += test_pattern_cached(&pattern[towel.len()..], towels, memos);
        }
    }

    memos.insert(pattern, count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            next_towel_options("r", &["b", "br", "bwu", "g", "gb", "r", "rb", "wr"]).collect_vec(),
            &["r"]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            next_towel_options(
                "gugrrrugwbugrrwbbbbggrwbuurgbbgurururuburrbwgugruwubwrggwr",
                &["gubrub", "gubwu", "gug", "guggg", "gur"]
            )
            .collect_vec(),
            &["gug"]
        )
    }
}
