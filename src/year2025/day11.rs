//
// Copyright (c) 2025 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;
use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};

// fn print_graphviz(graph: &HashMap<&str, Vec<&str>>) {
//     println!("digraph G {{");
//     for (parent, children) in graph {
//         println!("    {} -> {{ {} }};", parent, children.join(", "));
//     }
//     println!("}}");
// }

// // On the actual input data, this completes quickly for a start of "you" but is impossibly slow for
// // a start of "svr".
// fn dfs_path_count(
//     children_map: &HashMap<&str, Vec<&str>>,
//     start: &str,
//     end: &str,
// ) -> Option<usize> {
//     if start == end {
//         return Some(1);
//     }
//     if children_map.get(start).is_none() {
//         return None;
//     }
//     Some(
//         children_map
//             .get(start)
//             .unwrap()
//             .iter()
//             .map(|&child| dfs_path_count(children_map, child, end).unwrap_or(0))
//             .sum(),
//     )
// }

fn longest_path_len(children_map: &HashMap<&str, Vec<&str>>, start: &str) -> usize {
    if children_map.get(start).is_none() {
        return 0;
    }
    let mut longest = 1usize;
    let mut children = children_map
        .get(start)
        .unwrap()
        .iter()
        .copied()
        .collect::<HashSet<&str>>();
    while !children.is_empty() {
        let next_children = children
            .iter()
            .flat_map(|c| match children_map.get(c) {
                Some(children) => children.iter().copied().collect_vec(),
                None => vec![],
            })
            .collect::<HashSet<&str>>();
        children = next_children;
        longest += 1;
    }

    longest
}

fn count_all_length_0_paths<'problem_input>(
    children_map: &HashMap<&'problem_input str, Vec<&'problem_input str>>,
) -> HashMap<(&'problem_input str, &'problem_input str), u64> {
    let mut out = HashMap::new();
    for (&parent, children) in children_map {
        out.insert((parent, parent), 1);
        for child in children {
            out.insert((child, child), 1);
        }
    }
    out
}

fn count_all_length_n_paths<'problem_input>(
    all_pairs: impl Iterator<Item = (&'problem_input str, &'problem_input str)>,
    children_map: &HashMap<&str, Vec<&'problem_input str>>,
    n_minus_one_path_counts: &HashMap<(&str, &str), u64>,
) -> HashMap<(&'problem_input str, &'problem_input str), u64> {
    let mut out: HashMap<(&'problem_input str, &'problem_input str), u64> = HashMap::new();
    for (src, dest) in all_pairs {
        if let Some(src_children) = children_map.get(src) {
            for child in src_children {
                if let Some(child_paths_to_dest) = n_minus_one_path_counts.get(&(child, dest)) {
                    let mut src_paths_to_dest = out.entry((src, dest)).or_insert(0);
                    *src_paths_to_dest += child_paths_to_dest;
                }
            }
        }
    }
    out
}

fn count_all_pair_paths<'problem_input>(
    all_pairs: impl Iterator<Item = (&'problem_input str, &'problem_input str)> + Clone,
    children_map: &HashMap<&'problem_input str, Vec<&'problem_input str>>,
) -> Vec<HashMap<(&'problem_input str, &'problem_input str), u64>> {
    let length_0_counts = count_all_length_0_paths(children_map);
    let mut out = vec![length_0_counts];
    for n in 1..longest_path_len(children_map, "svr") {
        out.push(count_all_length_n_paths(
            all_pairs.clone(),
            children_map,
            &out[n - 1],
        ));
    }
    out
}

pub fn day11(input: &str) -> SolutionResult {
    let lines = input.lines();

    let mut children_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut parents_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines {
        let parent = &line[..3];
        let children = line[5..].split(' ').collect::<Vec<&str>>();
        for &child in &children {
            parents_map
                .entry(child)
                .or_insert_with(Vec::new)
                .push(parent);
        }
        children_map.insert(parent, children);
    }

    // print_graphviz(&children_map);

    let mut all_nodes = parents_map
        .keys()
        .copied()
        .chain(children_map.keys().copied())
        .collect_vec();
    all_nodes.sort();
    all_nodes.dedup();
    let all_pairs = all_nodes
        .iter()
        .copied()
        .cartesian_product(all_nodes.iter().copied());

    let all_path_counts_by_length = count_all_pair_paths(all_pairs.clone(), &children_map);

    let count_paths = |src, dest| {
        all_path_counts_by_length
            .iter()
            .map(|map| map.get(&(src, dest)).unwrap_or(&0))
            .sum::<u64>()
    };

    let a = count_paths("you", "out");

    let svr_to_fft = count_paths("svr", "fft");
    let fft_to_dac = count_paths("fft", "dac");
    let dac_to_out = count_paths("dac", "out");

    let svr_to_dac = count_paths("svr", "dac");
    let dac_to_fft = count_paths("dac", "fft");
    let fft_to_out = count_paths("fft", "out");

    let b = max(
        svr_to_fft * fft_to_dac * dac_to_out,
        svr_to_dac * dac_to_fft * fft_to_out,
    );
    let b = if b == 0 {
        "No Solution".to_string()
    } else {
        b.to_string()
    };

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}
