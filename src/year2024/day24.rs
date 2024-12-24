// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::SolutionResult;
use itertools::Itertools;
use regex::Regex;
use std::cmp::PartialEq;
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString, strum_macros::Display, Eq, PartialEq)]
enum Op {
    #[strum(serialize = "AND")]
    And,
    #[strum(serialize = "OR")]
    Or,
    #[strum(serialize = "XOR")]
    Xor,
}

#[derive(Debug, Eq, PartialEq)]
struct Gate<'a> {
    op: Op,
    in1: &'a str,
    in2: &'a str,
}

pub fn day24(input: &str) -> SolutionResult {
    let (inputs, gates) = parse_input(input);

    let out_count = gates.keys().filter(|g| g.starts_with('z')).count();

    let str = (0..out_count)
        .map(|i| {
            if evaluate_gate(&format!("z{i:02}"), &gates, &inputs) {
                '1'
            } else {
                '0'
            }
        })
        .rev()
        .collect::<String>();
    let a = i64::from_str_radix(&str, 2).unwrap();

    // Print GraphViz DOT input which can be fed into GraphViz to visualize the graph of adders
    println!("digraph G {{");
    println!("  layout = dot;");
    for (out, gate) in &gates {
        println!("  {}_{}_{} -> {out};", gate.in1, gate.in2, gate.op);
        println!("  {} -> {}_{}_{};", gate.in1, gate.in1, gate.in2, gate.op);
        println!("  {} -> {}_{}_{};", gate.in2, gate.in1, gate.in2, gate.op);
    }
    println!("}}");

    for (out, gate) in &gates {
        if !out.starts_with('z') || *out == "z00" || out == gates.last_key_value().unwrap().0 {
            continue;
        }
        verify_adder(out, &gates, &inputs);
    }
    
    // XXX I do not actually have a general code solution to this problem yet.  I solved the problem
    // by running this code to:
    // 1) Print GraphViz DOT input to generate a graph of the adders
    // 2) Identify outputs which don't look like they're part of a proper adder structure
    // This pointed me to all four "problem" adders in the graph.  I then visually picked out some
    // gates that looked like the ones that needed to be swapped and they were correct.
    
    // TODO: take the outputs identified by verify_adder() as incorrect, and for each of them, test
    // swapping with every other output to find the swap that resolves the problem.  There are few
    // enough "problem" outputs that this should run in a reasonable amount of time.

    SolutionResult::new(a, "")
}

fn evaluate_gate(out: &str, gates: &BTreeMap<&str, Gate>, inputs: &BTreeMap<&str, bool>) -> bool {
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

// Look for problems with the structure of the given output's adder
fn verify_adder(out: &str, gates: &BTreeMap<&str, Gate>, inputs: &BTreeMap<&str, bool>) -> bool {
    assert!(out.starts_with('z'));
    assert_ne!(out, "z00");
    assert_ne!(out, *gates.last_key_value().unwrap().0);
    
    let gate = gates.get(out).unwrap();

    if gate.op != Op::Xor {
        println!("{out}: has wrong input gate: {}", gate.op);
        return false;
    }
    if inputs.contains_key(gate.in1) {
        println!("{out}: Input {} should not be connected to {}'s XOR gate", gate.in1, out);
        return false;
    }
    if inputs.contains_key(gate.in2) {
        println!("{out}: Input {} should not be connected to {}'s XOR gate", gate.in2, out);
        return false;
    }
    // One of the XOR inputs is a carry bit (OR) from the previous adder
    // The other is a XOR output from the two adder input bits
    let mut carry = gate.in1;
    let mut in_xor = gate.in2;
    if gates.get(carry).unwrap().op != Op::Or {
        carry = gate.in2;
        in_xor = gate.in1;
    }
    if gates.get(carry).unwrap().op != Op::Or {
        // FIXME: z01 carry bit is an AND, not an OR
        println!("{}: Could not identify a carry bit for output", out);
        return false;
    }
    if gates.get(in_xor).unwrap().op != Op::Xor {
        println!("{}: Could not identify a adder input XOR for output", out);
        return false;
    }
    
    let in_xor_gate = gates.get(in_xor).unwrap();
    if in_xor_gate.in1[1..] != out[1..] {
        println!("{}: has incorrect input bit {}", out, in_xor_gate.in1);
    }
    if in_xor_gate.in2[1..] != out[1..] {
        println!("{}: has incorrect input bit {}", out, in_xor_gate.in2);
    }
    
    true
}

fn parse_input(input: &str) -> (BTreeMap<&str, bool>, BTreeMap<&str, Gate>) {
    let lines = input.lines().collect_vec();
    let split = lines.iter().position(|l| l.is_empty()).unwrap();

    let input_regex = Regex::new(r"(.+): ([0|1])").unwrap();
    let inputs = lines[..split]
        .iter()
        .map(|s| {
            let (_, [a, b]) = input_regex.captures(s).unwrap().extract();
            (a, b.parse::<i32>().unwrap() != 0)
        })
        .collect::<BTreeMap<_, _>>();

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
        .collect::<BTreeMap<_, _>>();
    (inputs, gates)
}
