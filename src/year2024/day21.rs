// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::iter;
use std::slice::SplitInclusive;

const NUM_KEYPAD_BYTES: &[u8] = b"789\n456\n123\n#0A\n";
const DIR_KEYPAD_BYTES: &[u8] = b"#^A\n<v>>\n";

pub fn day21(input: &str) -> SolutionResult {
    let door_codes = input.lines().map(|l| l.as_bytes()).collect_vec();
    let num_keypad = Grid::from_u8(NUM_KEYPAD_BYTES);
    let dir_keypad = Grid::from_u8(DIR_KEYPAD_BYTES);

    let num_key_paths = find_keypad_paths(&num_keypad);
    let dir_key_paths = find_keypad_paths(&dir_keypad);

    let all_dir_key_paths_combinations = path_selections(&dir_key_paths);
    assert_eq!(all_dir_key_paths_combinations.len(), 16);

    let a: u64 = door_codes
        .iter()
        .map(|c| {
            code_complexity_tally(
                c,
                &num_key_paths,
                &dir_key_paths,
                &all_dir_key_paths_combinations,
                2,
            )
        })
        .sum();

    let b: u64 = door_codes
        .iter()
        .map(|c| {
            code_complexity_tally(
                c,
                &num_key_paths,
                &dir_key_paths,
                &all_dir_key_paths_combinations,
                25,
            )
        })
        .sum();
    
    SolutionResult::new(a, b)
}

const CARDINALS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn cardinal_to_u8(cardinal: Vec2<isize>) -> u8 {
    match cardinal {
        Vec2 { x: 0, y: 1 } => b'v',
        Vec2 { x: 1, y: 0 } => b'>',
        Vec2 { x: 0, y: -1 } => b'^',
        Vec2 { x: -1, y: 0 } => b'<',
        _ => panic!(),
    }
}

fn neighbors<'a>(
    grid: &'a Grid<u8>,
    pos: &'a Vec2<isize>,
) -> impl Iterator<Item = Vec2<isize>> + 'a {
    CARDINALS
        .into_iter()
        .map(|c| *pos + c)
        .filter(|c| grid.area().contains(*c) && grid[*c] != b'#')
}

fn bfs(grid: &Grid<u8>, start: Vec2<isize>) -> Grid<Vec<Vec2<isize>>> {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut predecessors = Grid::<Vec<Vec2<isize>>>::new(grid.width(), grid.height());
    let mut distances = Grid::<i64>::new(grid.width(), grid.height());
    distances.data_mut_slice().fill(i64::MAX);
    distances[start] = 0;

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();

        for neighbor in neighbors(grid, &v) {
            if distances[neighbor] > distances[v] + 1 {
                distances[neighbor] = distances[v] + 1;
                predecessors[neighbor] = vec![v];
                queue.push_back(neighbor);
            } else if distances[neighbor] == distances[v] + 1 {
                predecessors[neighbor].push(v);
            }
        }
    }
    predecessors
}

fn find_paths(
    predecessors: &Grid<Vec<Vec2<isize>>>,
    start: Vec2<isize>,
    end: Vec2<isize>,
) -> Vec<Vec<Vec2<isize>>> {
    if start == end {
        return vec![vec![start]];
    }
    let mut paths = vec![];
    for pred in predecessors[end].clone() {
        for path in &mut find_paths(predecessors, start, pred) {
            path.push(end);
            paths.push(path.clone());
        }
    }
    paths
}

fn path_to_u8(path: &[Vec2<isize>]) -> Vec<u8> {
    path.windows(2)
        .map(|w| cardinal_to_u8((w[1] - w[0])))
        .chain(iter::once(b'A'))
        .collect_vec()
}

fn grid_find(grid: &Grid<u8>, target: u8) -> Option<Vec2<isize>> {
    grid.area()
        .all_points()
        .find(|&point| grid[point] == target)
        .map(Vec2::from_tuple)
}

fn find_keypad_paths(grid: &Grid<u8>) -> BTreeMap<(u8, u8), Vec<Vec<u8>>> {
    let keys = grid
        .data_slice()
        .iter()
        .filter(|b| **b != b'#')
        .collect_vec();
    let mut paths = BTreeMap::<(u8, u8), Vec<Vec<u8>>>::new();
    for button1 in &keys {
        let pos1 = grid_find(grid, **button1).unwrap();
        let predecessors = bfs(grid, pos1);
        for button2 in &keys {
            let pos2 = grid_find(grid, **button2).unwrap();
            let entry = paths.entry((grid[pos1], grid[pos2])).or_default();
            for path in find_paths(&predecessors, pos1, pos2) {
                entry.push(path_to_u8(&path));
            }
        }
    }

    // Filter out paths with more turns than necessary
    // These appear to never be the best path, and their compound expansions are dramatically longer
    for path_list in paths.values_mut() {
        let least_turns = path_list
            .iter()
            .map(|path| path.chunk_by(|a, b| a == b).count())
            .min()
            .unwrap();
        path_list.retain(|path| path.chunk_by(|a, b| a == b).count() == least_turns);
    }

    paths
}

fn path_selections(paths: &BTreeMap<(u8, u8), Vec<Vec<u8>>>) -> Vec<BTreeMap<(u8, u8), Vec<u8>>> {
    paths
        .values()
        .multi_cartesian_product()
        .map(|paths2| {
            iter::zip(
                paths.keys().copied(),
                paths2.iter().cloned().cloned().collect_vec(),
            )
            .collect::<BTreeMap<(u8, u8), Vec<u8>>>()
        })
        .collect_vec()
}

/// Split a sequence of button presses into groups ending in 'A'
fn split_button_presses(button_presses: &[u8]) -> SplitInclusive<'_, u8, fn(&u8) -> bool> {
    button_presses.split_inclusive(|&b| b == b'A')
}

fn tally(button_presses: &[u8]) -> Tally {
    let mut counts = BTreeMap::new();
    for seq in split_button_presses(button_presses) {
        *counts.entry(seq.to_vec()).or_insert(0) += 1;
    }
    counts
}

fn expand_tally(tally_in: &Tally, key_paths: &BTreeMap<(u8, u8), Vec<u8>>) -> Tally {
    let mut tally_out = Tally::new();
    for (path, count) in tally_in {
        let expanded = expand_code_once_best(path, key_paths);
        let expanded_tally = tally(&expanded);
        for (path2, count2) in &expanded_tally {
            *tally_out.entry(path2.clone()).or_default() += count * count2;
        }
    }

    tally_out
}

fn expand_tally_search(
    tally_in: &Tally,
    next_paths: &BTreeMap<(u8, u8), Vec<Vec<u8>>>,
    dir_paths: &BTreeMap<(u8, u8), Vec<Vec<u8>>>,
) -> BTreeSet<Tally> {
    let mut expansions_out = tally_in
        .iter()
        .map(|(path, count)| {
            expand_code_once(path, next_paths, dir_paths)
                .iter()
                .map(|path| {
                    let mut tally = tally(path);
                    for count2 in tally.values_mut() {
                        *count2 *= count;
                    }
                    tally
                })
                .collect_vec()
        })
        .multi_cartesian_product()
        .map(|tallies| {
            let mut out = Tally::new();
            for tally in &tallies {
                for (path2, count2) in tally {
                    *out.entry(path2.clone()).or_default() += count2;
                }
            }
            out
        })
        .collect::<BTreeSet<Tally>>();

    let min = expansions_out.iter().map(tally_length).min().unwrap();
    expansions_out.retain(|tally| tally_length(tally) == min);

    expansions_out
}

fn expand_tallies_search<'a>(
    tallies_in: impl IntoIterator<Item = &'a Tally> + std::fmt::Debug + std::clone::Clone,
    next_paths: &BTreeMap<(u8, u8), Vec<Vec<u8>>>,
    dir_paths: &BTreeMap<(u8, u8), Vec<Vec<u8>>>,
) -> BTreeSet<Tally> {
    let tallies_in = tallies_in.into_iter();
    let mut expansions_out = tallies_in
        .flat_map(|tally| expand_tally_search(tally, next_paths, dir_paths))
        .collect::<BTreeSet<Tally>>();

    let min = expansions_out.iter().map(tally_length).min().unwrap();
    expansions_out.retain(|tally| tally_length(tally) == min);

    expansions_out
}

fn tally_length(tally_in: &Tally) -> u64 {
    tally_in
        .iter()
        .map(|(path, count)| path.len() as u64 * count)
        .sum()
}

fn expand_code_tallies(
    code: &[u8],
    num_key_paths: &BTreeMap<(u8, u8), Vec<Vec<u8>>>,
    dir_key_paths: &BTreeMap<(u8, u8), Vec<Vec<u8>>>,
    all_dir_paths_combos: &[BTreeMap<(u8, u8), Vec<u8>>],
    iterations: usize,
) -> u64 {
    let mut tallies: BTreeSet<Tally> = BTreeSet::new();
    tallies.insert(tally(code));
    tallies = expand_tallies_search(&tallies, num_key_paths, dir_key_paths);

    all_dir_paths_combos
        .iter()
        .map(|combo| {
            tallies
                .iter()
                .map(|tally| {
                    let mut expanded = tally.clone();
                    for _ in 0..iterations {
                        expanded = expand_tally(&expanded, combo);
                    }
                    tally_length(&expanded)
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

type Tally = BTreeMap<Vec<u8>, u64>;

fn simple_expand_len(code: &[u8], key_paths: &BTreeMap<(u8, u8), Vec<Vec<u8>>>) -> usize {
    let code = prepend_a(code).collect_vec();
    code.iter()
        .tuple_windows::<(_, _)>()
        .flat_map(|(p1, p2)| key_paths[&(*p1, *p2)].first().unwrap())
        .count()
}

fn expand_code_once(
    code: &[u8],
    key_paths: &BTreeMap<(u8, u8), Vec<Vec<u8>>>,
    next_key_paths: &BTreeMap<(u8, u8), Vec<Vec<u8>>>,
) -> Vec<Vec<u8>> {
    let code = prepend_a(code).collect_vec();
    let mut expansions: Vec<Vec<u8>> = vec![vec![]];
    for (a, b) in code.iter().tuple_windows::<(_, _)>() {
        let mut new_expansions = vec![];
        for expansion in &expansions {
            for append in &key_paths[&(*a, *b)] {
                let mut new_expansion = expansion.clone();
                new_expansion.append(&mut append.clone());
                new_expansions.push(new_expansion);
            }
        }
        expansions = new_expansions;
    }

    let min = expansions
        .iter()
        .map(|a| simple_expand_len(a, next_key_paths))
        .min()
        .unwrap();
    expansions.retain(|a| simple_expand_len(a, next_key_paths) == min);

    expansions
}

fn expand_code_once_best(code: &[u8], key_paths: &BTreeMap<(u8, u8), Vec<u8>>) -> Vec<u8> {
    let code = prepend_a(code).collect_vec();
    let mut expansion: Vec<u8> = vec![];
    for (a, b) in code.iter().tuple_windows::<(_, _)>() {
        expansion.append(&mut key_paths[&(*a, *b)].clone());
    }
    expansion
}

fn prepend_a(button_presses: &[u8]) -> impl Iterator<Item = u8> + use<'_> {
    iter::once(b'A').chain(button_presses.iter().copied())
}

fn code_complexity_tally(
    code: &[u8],
    num_key_paths: &BTreeMap<(u8, u8), Vec<Vec<u8>>>,
    dir_key_paths: &BTreeMap<(u8, u8), Vec<Vec<u8>>>,
    all_dir_paths_combos: &[BTreeMap<(u8, u8), Vec<u8>>],
    iterations: usize,
) -> u64 {
    let expanded_len = expand_code_tallies(
        code,
        num_key_paths,
        dir_key_paths,
        all_dir_paths_combos,
        iterations,
    );
    let numeric_part: u64 = std::str::from_utf8(&code[0..3]).unwrap().parse().unwrap();
    expanded_len * numeric_part
}
