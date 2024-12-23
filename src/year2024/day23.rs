// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use std::collections::{BTreeMap, BTreeSet};
use itertools::Itertools;
use adventofcode_rust::aoc::SolutionResult;

pub fn day23(input: &str) -> SolutionResult {
    let pairs = input.lines().map(|line| (&line[0..2], &line[3..5])).collect_vec();

    let mut edges = BTreeMap::new();
    let mut nodes = vec![];
    for (a, b) in &pairs {
        edges.entry(*a).or_insert_with(BTreeSet::new).insert(*b);
        edges.entry(*b).or_insert_with(BTreeSet::new).insert(*a);
        nodes.push(*a);
        nodes.push(*b);
    }
    nodes.sort_unstable();
    nodes.dedup();

    let mut triples = BTreeSet::new();
    for a in &nodes {
        for (b,c) in edges.get(a).unwrap().iter().tuple_combinations() {
            if edges[b].contains(c) {
                let mut triple = [a, *b, *c];
                triple.sort_unstable();
                triples.insert(triple);
            }
        }
    }

    let a = triples.iter().filter(|triple| has_computer_starting_with_t(triple)).count();

    let mut edges_including_self = edges.clone();
    for node in &nodes {
        edges_including_self.get_mut(node).unwrap().insert(node);
    }

    let b = solve_part2(&nodes, edges_including_self);

    SolutionResult::new(a,b)
}

fn solve_part2(nodes: &Vec<&str>, edges_including_self: BTreeMap<&str, BTreeSet<&str>>) -> String {
    let max_network_size = edges_including_self.first_key_value().unwrap().1.len();
    for _ in (0..max_network_size).rev() {
        for node1 in nodes {
            for candidate_party in edges_including_self.get(node1).unwrap().iter().combinations(13) {
                let matched = candidate_party.iter().all(|node2| {
                    candidate_party.iter().all(|b| edges_including_self[*node2].contains(*b))
                });
                if matched {
                    return candidate_party.iter().join(",");
                }
            }
        }
    }
    panic!("No solution found");
}

fn has_computer_starting_with_t(triple: &[&str; 3]) -> bool{
    triple.iter().any(|s| s.starts_with('t'))
}
