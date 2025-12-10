//
// Copyright (c) 2025 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;
use bitvec::prelude::*;
use itertools::Itertools;
use regex::Regex;

type LightSet = u16;
const EMPTY_LIGHTSET: LightSet = 0u16;

fn set_bit(val: &mut u16, idx: usize, set: bool) -> () {
    if set {
        *val |= 1 << idx;
    } else {
        *val &= !(1 << idx);
    }
}

#[derive(Debug)]
struct Machine {
    lights: LightSet,
    buttons: Vec<LightSet>,
    joltages: Vec<u8>,
}

impl Machine {
    fn solve(&self) -> u32 {
        let mut best: Option<u32> = None;
        for mut i in 1..2u32.pow(self.buttons.len() as u32) {
            let mut button_idx_bits = i;
            let mut combined_buttons = EMPTY_LIGHTSET;

            // Iterate button indexes
            for bit_idx in 0..16 {
                let present = button_idx_bits & 1 == 1;
                if present {
                    combined_buttons ^= self.buttons[bit_idx];
                }
                button_idx_bits >>= 1;
                if button_idx_bits == 0 {
                    break;
                }
            }
            if combined_buttons == self.lights {
                let count = i.count_ones();
                best = match best {
                    Some(b) => Some(b.min(count)),
                    None => Some(count),
                };
            }
        }
        best.unwrap()
    }
}

impl From<&str> for Machine {
    fn from(line: &str) -> Self {
        let input_regex = Regex::new(r"\[(.+)] (.+) \{(.+)}").unwrap();
        let (_, [a, b, c]) = input_regex.captures(line).unwrap().extract();

        let mut lights = EMPTY_LIGHTSET;
        for (i, c) in a.chars().enumerate() {
            set_bit(&mut lights, i, c == '#');
        }
        let buttons = buttons_from_str(b);

        // TODO: Joltages

        Machine {
            lights,
            buttons,
            joltages: Vec::new(),
        }
    }
}

fn button_from_str(s: &str) -> LightSet {
    let mut output = EMPTY_LIGHTSET;
    for num in s[1..s.len() - 1].split(',') {
        set_bit(&mut output, num.parse::<usize>().unwrap(), true);
    }
    output
}

fn buttons_from_str(s: &str) -> Vec<LightSet> {
    s.split(' ').map(button_from_str).collect_vec()
}

pub fn day10(input: &str) -> SolutionResult {
    let lines = input.lines();
    let machines = lines.map(Machine::from).collect::<Vec<_>>();

    // for (i, machine) in machines.iter().enumerate() {
    //     println!("{}: {}: {}", i, machine.lights, machine.solve());
    // }

    let a = machines.iter().map(Machine::solve).sum::<u32>();
    let b = "";

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}
