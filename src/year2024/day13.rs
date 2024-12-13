//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{SolutionResult, Vec2};
use itertools::Itertools;

type SixNums = (f64, f64, f64, f64, f64, f64);

#[derive(Clone, Debug)]
struct Machine {
    a: Vec2<f64>,
    b: Vec2<f64>,
    prize: Vec2<f64>,
}

pub fn day13(input: &str) -> SolutionResult {
    let machines = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f64>().unwrap())
        .tuples::<SixNums>()
        .map(parse_machine)
        .collect_vec();

    let a = solve(&machines);

    let adjusted_machines = machines
        .iter()
        .map(|m| Machine {
            prize: m.prize + (10000000000000.0, 10000000000000.0),
            ..m.clone()
        })
        .collect_vec();

    let b = solve(&adjusted_machines);

    SolutionResult::new(a, b)
}

fn solve(machines: &[Machine]) -> i64 {
    let a = machines
        .iter()
        .filter_map(solve_machine)
        .map(token_cost)
        .sum::<i64>();
    a
}

fn parse_machine(nums: SixNums) -> Machine {
    Machine {
        a: Vec2::new(nums.0, nums.1),
        b: Vec2::new(nums.2, nums.3),
        prize: Vec2::new(nums.4, nums.5),
    }
}

// Return an int only if the float really was approximately an int
fn try_float_to_int(float: f64) -> Option<i64> {
    let round_trip = ((float + 0.5) as i64) as f64;
    match (float - round_trip).abs() < 0.01 {
        true => Some((float + 0.5) as i64),
        false => None,
    }
}

fn solve_machine(machine: &Machine) -> Option<(i64, i64)> {
    let mut machine = machine.clone();

    // This is a solution to a system of linear equations by row reduction
    // a*A.x + b*B.x = prize.x
    // a*A.y + b*B.y = prize.y
    let x_div = machine.a.x;
    machine.a.x /= x_div;
    machine.b.x /= x_div;
    machine.prize.x /= x_div;

    let multiple = machine.a.y;
    machine.a.y -= machine.a.x * multiple;
    machine.b.y -= machine.b.x * multiple;
    machine.prize.y -= machine.prize.x * multiple;

    let y_div = machine.b.y;
    machine.b.y /= y_div;
    machine.prize.y /= y_div;
    let b = try_float_to_int(machine.prize.y);

    let multiple = machine.b.x;
    machine.b.x -= machine.b.y * multiple;
    machine.prize.x -= machine.prize.y * multiple;
    let a = try_float_to_int(machine.prize.x);

    match (a, b) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

fn token_cost((a, b): (i64, i64)) -> i64 {
    a * 3 + b
}
