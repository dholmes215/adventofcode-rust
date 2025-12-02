//
// Copyright (c) 2025 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;
use itertools::Itertools;

pub fn day02(input: &str) -> SolutionResult {
    let number_strs = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let chunks = number_strs.iter().chunks(2);
    let all_numbers_in_ranges = chunks.into_iter().flat_map(|chunk| {
        let pair = chunk.collect_tuple::<(_, _)>().unwrap();
        pair.0.parse::<i64>().unwrap()..pair.1.parse::<i64>().unwrap()
    }).collect::<Vec<i64>>();

    let a = all_numbers_in_ranges
        .iter()
        .filter(|id| is_invalid_num_a(**id))
        .sum::<i64>();

    let b = all_numbers_in_ranges
        .iter()
        .filter(|id| is_invalid_num_b(**id))
        .sum::<i64>();

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}

fn is_invalid_str_a(id: &str) -> bool {
    let half = id.len() / 2;
    id[0..half] == id[half..]
}

fn is_invalid_num_a(id: i64) -> bool {
    is_invalid_str_a(&id.to_string())
}

pub fn is_invalid_str_b(id: String) -> bool {
    for chunk_size in 1..id.len() / 2 + 1 {
        let chunks = id.chars().chunks(chunk_size);
        let mut iter = chunks.into_iter();
        let first = iter.next().unwrap().collect::<String>();
        if iter
            .all(|id| id.eq(first.chars()))
        {
            return true;
        }
    }
    false
}

fn is_invalid_num_b(id: i64) -> bool {
    is_invalid_str_b(id.to_string())
}
