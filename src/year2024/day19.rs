// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use std::collections::HashMap;
use std::iter::Filter;
use std::slice::Iter;
use adventofcode_rust::aoc::SolutionResult;
use itertools::Itertools;

pub fn day19(input: &str) -> SolutionResult {
    let mut lines = input.lines();
    let mut towels = lines.next().unwrap().split(", ").collect_vec();
    towels.sort();
    let patterns = lines.skip(1).collect_vec();

    println!("Towels: {:?}", towels);

    for pattern in &patterns {
        println!("{:?}: {}", pattern, test_pattern(pattern, &towels));
    }

    let a = patterns.iter().filter(|p| test_pattern(p, &towels)).count();

    SolutionResult::new(a, 0)
}

// Return the subset of the given towels that could be the first towel of the given pattern
fn next_towel_options<'a>(pattern: &str, sorted_towels: &'a [&'a str]) -> Vec<&'a str>  {
    // assert!(sorted_towels.is_sorted());
    // let end = sorted_towels.partition_point(|towel| *towel <= pattern);
    // let matching_towels = &sorted_towels[..end];
    // let end = matching_towels.len() - matching_towels.iter().rev().position(|towel| pattern.starts_with(*towel)).unwrap_or(0);
    // let matching_towels = &sorted_towels[..end];
    // let begin = matching_towels.partition_point(|towel| !pattern.starts_with(*towel));
    // &matching_towels[begin..]
    sorted_towels.iter().filter(|towel| pattern.starts_with(*towel)).map(|towel| *towel).collect_vec()
}

fn test_pattern(pattern: &str, towels: &[&str]) -> bool {
    let mut cache = HashMap::new();
    test_pattern_cached(pattern, towels, &mut cache)
}

fn test_pattern_cached(pattern: &str, towels: &[&str], memos: &mut HashMap<String, bool>) -> bool {
    println!("Testing: {:?}", pattern);
    if let Some(result) = memos.get(pattern) {
        println!("Cached result for {pattern}: {result}");
        return *result;
    }
    let mut matching_towels = next_towel_options(pattern, towels);
    // println!("Matching towels: {:?}", matching_towels);

    // Return early if there's an exact match
    for towel in matching_towels.clone() {
        // println!("Comparing {} with {}: {}", pattern, *towel, pattern == *towel);
        if *pattern == *towel {
            memos.insert(pattern.to_string(), true);
            return true;
        }
    }

    for towel in matching_towels {
        if pattern.starts_with(towel) && test_pattern_cached(&pattern[towel.len()..], towels, memos) {
            memos.insert(pattern.to_string(), true);
            return true;
        }
    }

    memos.insert(pattern.to_string(), false);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            next_towel_options("r", &["b", "br", "bwu", "g", "gb", "r", "rb", "wr"]),
            &["r"]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            next_towel_options(
                "gugrrrugwbugrrwbbbbggrwbuurgbbgurururuburrbwgugruwubwrggwr",
                &["gubrub", "gubwu", "gug", "guggg", "gur"]
            ),
            &["gug"]
        )
    }
}
