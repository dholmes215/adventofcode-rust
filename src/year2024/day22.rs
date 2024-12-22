// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::SolutionResult;
use itertools::Itertools;
use rayon::prelude::*;

pub fn day22(input: &str) -> SolutionResult {
    let initial_numbers = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect_vec();

    let computed_secrets = initial_numbers
        .iter()
        .map(|n| {
            itertools::iterate(*n, |n| next_secret(*n))
                .take(2001)
                .collect_vec()
        })
        .collect_vec();
    let a = computed_secrets
        .iter()
        .map(|sequence| sequence[2000])
        .sum::<i64>();

    let prices = computed_secrets
        .iter()
        .map(|sequence| sequence.iter().map(|n| n % 10).collect_vec())
        .collect_vec();

    let price_changes = prices
        .iter()
        .map(|sequence| {
            sequence
                .iter()
                .tuple_windows::<(_, _)>()
                .map(|(a, b)| b - a)
                .collect_vec()
        })
        .collect_vec();

    let change_sequence_candidates = std::iter::repeat_n((-9..=9), 4)
        .multi_cartesian_product()
        .filter(|sequence| sequence.iter().sum::<i64>() >= -9 && sequence.iter().sum::<i64>() <= 9)
        .collect_vec();
    let change_windows = price_changes
        .iter()
        .map(|sequence| sequence.windows(4).collect_vec())
        .collect_vec();
    let change_windows_with_prices = std::iter::zip(change_windows, &prices)
        .map(|(change_sequence, prices)| {
            std::iter::zip(change_sequence, &prices[4..]).collect_vec()
        })
        .collect_vec();

    let b = change_sequence_candidates
        .par_iter()
        .map(|candidate| {
            change_windows_with_prices
                .iter()
                .map(
                    |sequence| match sequence.iter().find(|(s, _)| *s == candidate.as_slice()) {
                        None => 0i64,
                        Some((_, p)) => **p,
                    },
                )
                .sum::<i64>()
        })
        .max()
        .unwrap();

    SolutionResult::new(a, b)
}

fn next_secret(mut secret: i64) -> i64 {
    let product = secret * 64;
    secret ^= product; // mix
    secret %= 16777216; // prune
    let quotient = secret / 32;
    secret ^= quotient; // mix
    secret %= 16777216; // prune
    let product = secret * 2048;
    secret ^= product; // mix
    secret %= 16777216; // prune
    secret
}
