// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{Grid, SolutionResult};
use arrayvec::ArrayVec;
use itertools::Itertools;

pub fn day25(input: &str) -> SolutionResult {
    assert!(!input.as_bytes().contains(&b'\r'));
    let grids = input.as_bytes().chunks(43).map(Grid::from_u8).collect_vec();

    let mut locks = vec![];
    let mut keys = vec![];
    for grid in &grids {
        let heights: ArrayVec<_, 5> = grid
            .cols()
            .map(|col| col.filter(|b| **b == b'#').count())
            .collect();
        if grid[(0, 0)] == b'#' {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let a = locks
        .iter()
        .cartesian_product(keys.iter())
        .map(|(lock, key)| {
            std::iter::zip(lock, key)
                .map(|(l, k)| l + k)
                .collect::<ArrayVec<_, 5>>()
        })
        .filter(|heights| heights.iter().all(|h| *h <= 7))
        .count();

    SolutionResult::new(a, "🎄")
}
