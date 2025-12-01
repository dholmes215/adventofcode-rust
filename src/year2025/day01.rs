//
// Copyright (c) 2025 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;

// To avoid having to deal with wrapping when underflowing, just use a big
// number and only look at the last two digits to know the actual dial value.
const DIAL_START: i64 = 1000000050;
const DIAL_MAX: i64 = 100;

struct Rotation {
    direction: char,
    turns: i64,
}

fn parse_rotation(input: &str) -> Rotation {
    Rotation {
        direction: input.chars().next().unwrap(),
        turns: input[1..].parse::<i64>().unwrap(),
    }
}

struct RotateResult {
    dial: i64,
    zero_count: u64,
}

fn rotate_dial(dial: i64, r: Rotation) -> RotateResult {
    let mut zero_count: u64 = 0;
    let sign: i64 = if r.direction == 'L' { -1 } else { 1 };
    let new_dial = dial + sign * r.turns;
    let adjustment = if r.direction == 'L' {
        ((new_dial % DIAL_MAX == 0) as i64) - ((dial % DIAL_MAX == 0) as i64)
    } else {
        0
    };
    zero_count += (((new_dial / DIAL_MAX) - (dial / DIAL_MAX)).abs() + adjustment) as u64;

    RotateResult {
        dial: new_dial,
        zero_count,
    }
}

pub fn day01(input: &str) -> SolutionResult {
    let mut p1_zero_count: u64 = 0;
    let mut p2_zero_count: u64 = 0;
    let mut dial = DIAL_START;
    for r in input.lines().map(parse_rotation) {
        let RotateResult {
            dial: new_dial,
            zero_count
        } = rotate_dial(dial, r);
        dial = new_dial;
        p2_zero_count += zero_count;
        if dial % DIAL_MAX == 0 {
            p1_zero_count += 1;
        }
    }

    SolutionResult {
        a: p1_zero_count.to_string(),
        b: p2_zero_count.to_string(),
    }
}
