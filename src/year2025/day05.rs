//
// Copyright (c) 2025 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;
use itertools::Itertools;
use std::collections::BTreeMap;

pub fn day05(input: &str) -> SolutionResult {
    let ProcessedInput {
        fresh_intervals,
        ingredients,
    } = process_input(input);

    let is_fresh_predicate = |id: &&IngredientId| is_fresh(&fresh_intervals, **id);
    let a = ingredients.iter().filter(is_fresh_predicate).count();
    let b = count_fresh(&fresh_intervals);

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}

type IngredientId = u64;
type IntervalsMap = BTreeMap<IngredientId, bool>;

struct ProcessedInput {
    fresh_intervals: IntervalsMap,
    ingredients: Vec<IngredientId>,
}

fn is_fresh(intervals: &IntervalsMap, id: IngredientId) -> bool {
    *intervals.range(..=id).next_back().unwrap().1
}

fn process_input(input: &str) -> ProcessedInput {
    let mut fresh_intervals = BTreeMap::new();
    fresh_intervals.insert(0, false);

    let mut iter = input.lines();

    for line in iter.by_ref() {
        if line.is_empty() {
            break;
        }

        let (left_inclusive, right_inclusive) = line
            .split('-')
            .map(|s| s.parse::<IngredientId>().unwrap())
            .collect_tuple::<(_, _)>()
            .unwrap();

        let after_right_already_fresh = is_fresh(&fresh_intervals, right_inclusive + 1);

        // Remove any existing entries in this interval
        let keys_to_remove = fresh_intervals
            .range(left_inclusive..=right_inclusive)
            .map(|(k, v)| *k)
            .collect_vec();
        for key in keys_to_remove {
            fresh_intervals.remove(&key);
        }

        if !is_fresh(&fresh_intervals, left_inclusive) {
            fresh_intervals.insert(left_inclusive, true);
        }
        if !after_right_already_fresh {
            fresh_intervals.insert(right_inclusive + 1, false);
        } else {
            fresh_intervals.remove(&(right_inclusive + 1));
        }
    }

    let ingredients = iter
        .map(|line| line.parse::<IngredientId>().unwrap())
        .collect();

    ProcessedInput {
        fresh_intervals,
        ingredients,
    }
}

fn count_fresh(fresh_intervals: &IntervalsMap) -> usize {
    let mut iter = fresh_intervals.iter().peekable();
    if !iter.peek().unwrap().1 {
        iter.next();
    }

    iter.map(|(k, _)| *k)
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.collect_tuple::<(_, _)>().unwrap())
        .map(|(l, r)| (r - l) as usize)
        .sum()
}
