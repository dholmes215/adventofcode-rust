//
// Copyright (c) 2025 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;
use itertools::Itertools;

pub fn day07(input: &str) -> SolutionResult {
    let mut input_lines: Vec<Vec<u8>> = input
        .lines()
        .map(|s| s.as_bytes().iter().copied().collect())
        .collect();

    let mut beam_timeline_counts: Vec<Vec<u64>> = input_lines
        .iter()
        .map(|vec| vec.iter().copied().map_into::<u64>().collect_vec())
        .collect();

    let mut split_count = 0;

    for line in beam_timeline_counts.iter_mut() {
        for c in line {
            if *c == b'S' as u64 {
                *c = 1;
            } else {
                *c = 0;
            }
        }
    }

    for (i1, i2) in (0..beam_timeline_counts.len()).tuple_windows() {
        let (count_line, count_next_line) = get_mut2(&mut beam_timeline_counts, i1, i2).unwrap();
        let input_next_line = &mut input_lines[i2];

        for (i, count) in count_line.iter().copied().enumerate() {
            if input_next_line[i] == b'^' {
                count_next_line[i - 1] += count;
                count_next_line[i + 1] += count;
                if count > 0 {
                    split_count += 1;
                }
            } else {
                count_next_line[i] += count;
            }
        }
    }

    let a = split_count;
    let b = beam_timeline_counts
        .last()
        .unwrap()
        .iter()
        .copied()
        .sum::<u64>();

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}

fn get_mut2<T>(v: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)> {
    if i == j {
        return None;
    }
    let (start, end) = if i < j { (i, j) } else { (j, i) };

    let (first, second) = v.split_at_mut(start + 1);
    Some((&mut first[start], &mut second[end - start - 1]))
}
