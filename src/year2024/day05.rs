//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::SolutionResult;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse_rule(input: &str) -> (i32, i32) {
    (
        input[3..5].parse::<i32>().unwrap(),
        input[0..2].parse::<i32>().unwrap(),
    )
}

fn check_update(update: &[i32], rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut previous_set = HashSet::new();
    for page in update {
        let prereqs = rules.get(page);
        if let Some(prereqs) = prereqs {
            for prereq in prereqs {
                if update.contains(prereq) && !previous_set.contains(prereq) {
                    return false;
                }
            }
        }
        previous_set.insert(page);
    }
    true
}

fn fix_update(update: &mut [i32], rules: &HashMap<i32, Vec<i32>>) {
    // XXX this is basically a gross bubble sort, but it was easy
    while !check_update(update, rules) {
        let mut previous_set = HashSet::new();
        for i in 0..update.len() {
            let page = update[i];
            let prereqs = rules.get(&page);
            if let Some(prereqs) = prereqs {
                for prereq in prereqs {
                    if update.contains(prereq) && !previous_set.contains(prereq) {
                        let subslice = &mut update[i..];
                        subslice.rotate_left(1);
                        break;
                    }
                }
            }
            previous_set.insert(page);
        }
    }
}

pub fn day05(input: &str) -> SolutionResult {
    let lines = input.split(|c| c == '\n' || c == '\r').collect_vec();
    let separator = lines.iter().position(|line| line.is_empty()).unwrap();
    let rule_lines = lines.iter().take(separator - 1);
    let update_lines = lines.iter().skip(separator).filter(|line| !line.is_empty());

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in rule_lines {
        let rule = parse_rule(line);
        rules.entry(rule.0).or_default().push(rule.1);
    }

    let updates = update_lines
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut bad_updates = Vec::new();

    let mut a = 0;
    for update in updates {
        if check_update(&update, &rules) {
            a += update[update.len() / 2];
        } else {
            bad_updates.push(update);
        }
    }

    let mut b = 0;
    for mut update in bad_updates {
        fix_update(update.as_mut_slice(), &rules);
        b += update[update.len() / 2];
    }

    SolutionResult::new(a, b)
}
