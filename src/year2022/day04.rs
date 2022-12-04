//
// Copyright (c) 2022 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;

use regex::Regex;

fn parse_line(line: &str) -> ((u8,u8),(u8,u8)) {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let captures = re.captures(line).unwrap();
    ((captures[1].parse::<u8>().unwrap(), captures[2].parse::<u8>().unwrap()),
     (captures[3].parse::<u8>().unwrap(), captures[4].parse::<u8>().unwrap()))
}

fn contains(a: (u8, u8), b: (u8, u8)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn either_contains(range: ((u8,u8),(u8,u8))) -> bool {
    contains(range.0, range.1) || contains(range.1, range.0)
}

fn contains_end(a: (u8, u8), b: (u8, u8)) -> bool {
    (a.0 >= b.0 && a.0 <= b.1) || (b.0 >= a.0 && b.0 <= a.1)
}

fn overlaps(range: ((u8,u8),(u8,u8))) -> bool {
    contains_end(range.0, range.1) || contains_end(range.1, range.0)
}

pub fn day04(input: &str) -> SolutionResult {
    let pairs =
        input.trim()
            .split("\n")
            .map(parse_line)
            .collect::<Vec<((u8,u8),(u8,u8))>>();

    SolutionResult {
        a: pairs.iter().filter(|x| either_contains(**x)).count().to_string(),
        b: pairs.iter().filter(|x| overlaps(**x)).count().to_string(),
    }
}
