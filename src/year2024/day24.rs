// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::SolutionResult;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
enum Op {
    #[strum(serialize = "AND")]
    And,
    #[strum(serialize = "OR")]
    Or,
    #[strum(serialize = "XOR")]
    Xor,
}

#[derive(Debug)]
struct Gate<'a> {
    op: Op,
    in1: &'a str,
    in2: &'a str,
}

pub fn day24(input: &str) -> SolutionResult {
    let (inputs, gates) = parse_input(input);

    let out_count = gates.keys().filter(|g| g.starts_with('z')).count();

    let str = (0..out_count).map(|i| if evaluate_gate(&format!("z{i:02}"), &gates, &inputs) {'1'} else {'0'}).rev().collect::<String>();
    let a = i64::from_str_radix(&str, 2).unwrap();
    SolutionResult::new(a, 0)
}

fn evaluate_gate(out: &str, gates: &HashMap<&str, Gate>, inputs: &HashMap<&str, bool>) -> bool {
    if out.starts_with('x') || out.starts_with('y') {
        return inputs[out];
    }

    let gate = gates.get(out).unwrap();
    match gate.op {
        Op::And => evaluate_gate(gate.in1, gates, inputs) && evaluate_gate(gate.in2, gates, inputs),
        Op::Or => evaluate_gate(gate.in1, gates, inputs) || evaluate_gate(gate.in2, gates, inputs),
        Op::Xor => evaluate_gate(gate.in1, gates, inputs) != evaluate_gate(gate.in2, gates, inputs),
    }
}

fn parse_input(input: &str) -> (HashMap<&str, bool>, HashMap<&str, Gate>) {
    let lines = input.lines().collect_vec();
    let split = lines.iter().position(|l| l.is_empty()).unwrap();

    let input_regex = Regex::new(r"(.+): ([0|1])").unwrap();
    let inputs = lines[..split]
        .iter()
        .map(|s| {
            let (_, [a, b]) = input_regex.captures(s).unwrap().extract();
            (a, b.parse::<i32>().unwrap() != 0)
        })
        .collect::<HashMap<_, _>>();

    let gates_regex = Regex::new(r"(.+) (.+) (.+) -> (.+)").unwrap();
    let gates = lines[split + 1..]
        .iter()
        .map(|s| {
            let (_, [in1, op, in2, out]) = gates_regex.captures(s).unwrap().extract();
            (
                out,
                Gate {
                    in1,
                    op: Op::from_str(op).unwrap(),
                    in2,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    (inputs, gates)
}
