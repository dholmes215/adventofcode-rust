//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::SolutionResult;
use itertools::Itertools;

fn safe<'a, I>(report: I) -> bool
where
    I: Sized + Clone + Iterator<Item = &'a i32>,
{
    report
        .clone()
        .tuple_windows()
        .map(|(p0, p1)| p0.cmp(&p1))
        .all_equal()
        && report.tuple_windows().all(|(p0, p1)| {
            let diff = p0.abs_diff(*p1);
            diff >= 1 && diff <= 3
        })
}

fn almost_safe(report: &Vec<i32>) -> bool {
    let mut modified_reports = (0..report.len())
        .into_iter()
        .map(|i| report.iter().take(i).chain(report.iter().skip(i + 1)));
    modified_reports.any(safe)
}

pub fn day02(input: &str) -> SolutionResult {
    let reports = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let a = reports.iter().filter(|r| safe(r.iter())).count();
    let b = reports.iter().filter(|r| almost_safe(&r)).count();

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}
