// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::iter;
use std::iter::{Chain, Copied, Once};
use std::slice::{Iter, SplitInclusive};
use rayon::iter::split;
use rayon::prelude::*;

const NUM_KEYPAD_BYTES: &[u8] = b"789\n456\n123\n#0A\n";
const DIR_KEYPAD_BYTES: &[u8] = b"#^A\n<v>>\n";

pub fn day21(input: &str) -> SolutionResult {
    let door_codes = input.lines().map(|l| l.as_bytes()).collect_vec();
    let num_keypad = Grid::from_u8(NUM_KEYPAD_BYTES);
    let dir_keypad = Grid::from_u8(DIR_KEYPAD_BYTES);

    let num_key_paths = find_keypad_paths(&num_keypad);
    let dir_key_paths = find_keypad_paths(&dir_keypad);

    // for ((a,b),v) in &dir_key_paths {
    //     println!("{},{}: {:?}", *a, *b, v);
    // }
    // println!("{dir_key_paths:#?}");

    // expand_code_test(&dir_key_paths, 3);
    // let a = 0;

    let a: u64 = door_codes
        .iter()
        .map(|c| code_complexity(c, &num_key_paths, &dir_key_paths))
        .sum();

    SolutionResult::new(a, 0)
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
            if distances[neighbor] > distances[v] + 1{
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

fn find_paths(predecessors: &Grid<Vec<Vec2<isize>>>, start: Vec2<isize>, end: Vec2<isize>) -> Vec<Vec<Vec2<isize>>> {
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

fn find_keypad_paths(grid: &Grid<u8>) -> HashMap<(u8, u8), Vec<Vec<u8>>> {
    let keys = grid
        .data_slice()
        .iter()
        .filter(|b| **b != b'#')
        .collect_vec();
    let mut paths = HashMap::<(u8, u8), Vec<Vec<u8>>>::new();
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
    paths
}

// fn find_best_expansions(grid: &Grid<u8>) -> HashMap<Vec<u8>, Vec<u8>> {
//     let keys = grid
//         .data_slice()
//         .iter()
//         .filter(|b| **b != b'#')
//         .collect_vec();
//     let mut paths = HashMap::<(u8, u8), Vec<Vec<u8>>>::new();
//     for button1 in &keys {
//         let pos1 = grid_find(grid, **button1).unwrap();
//         let predecessors = bfs(grid, pos1);
//         for button2 in &keys {
//             let pos2 = grid_find(grid, **button2).unwrap();
//             let entry = paths.entry((grid[pos1], grid[pos2])).or_default();
//             for path in find_paths(&predecessors, pos1, pos2) {
//                 entry.push(path_to_u8(&path));
//             }
//         }
//     }
//     paths
// }

fn expand_code_test(
    dir_key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>, i: usize) {

    let code = b"<vA";
    let mut expansions = vec![code.to_vec()];

    for i in 0..i {
        let mut new_expansions = vec![];
        for expansion in expansions {
            new_expansions.append(&mut expand_code_once(&expansion, dir_key_paths, dir_key_paths));
        }
        expansions = new_expansions;

        // let min = expansions.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap().len();
        // expansions = expansions.iter().filter(|a| a.len() == min).cloned().collect_vec();
    }

    for expansion in expansions {
        println!("{}", std::str::from_utf8(&expansion).unwrap());
    }
}

/// Split a sequence of button presses into groups ending in 'A'
fn split_button_presses(button_presses: &[u8]) -> SplitInclusive<'_, u8, fn(&u8) -> bool> {
    button_presses.split_inclusive(|&b| b == b'A')
}

fn tally_subsequences(button_presses: &[u8]) -> BTreeMap<Vec<u8>, usize> {
    let mut counts = BTreeMap::new();
    for seq in split_button_presses(button_presses) {
        *counts.entry(seq.to_vec()).or_insert(0) += 1;
    }
    counts
}

fn expand_code(
    code: &[u8],
    num_key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>,
    dir_key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>,
) -> Vec<u8> {
    let code = code.to_vec();
    let mut expansions = vec![code.to_vec()];

    let mut new_expansions = vec![];
    for expansion in expansions {
        new_expansions.append(&mut expand_code_once(&expansion, num_key_paths, dir_key_paths));
    }
    expansions = new_expansions;

    let mut new_expansions = vec![];
    for expansion in expansions {
        new_expansions.append(&mut expand_code_once(&expansion, dir_key_paths, dir_key_paths));
    }
    expansions = new_expansions;

    let mut new_expansions = vec![];
    for expansion in expansions {
        new_expansions.append(&mut expand_code_once(&expansion, dir_key_paths, dir_key_paths));
    }
    expansions = new_expansions;

    expansions.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap().clone()
}

fn remove_duplicate_tallies(mut tallies: Vec<BTreeMap<Vec<u8>, usize>>) -> Vec<BTreeMap<Vec<u8>, usize>> {
    tallies.sort();
    tallies.into_iter().unique().collect_vec()
}

fn expand_code2(
    code: &[u8],
    num_key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>,
    dir_key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>,
) -> Vec<u8> {
    let code = code.to_vec();
    let mut expansions = vec![code.to_vec()];

    let mut new_expansions = vec![];
    for expansion in expansions {
        new_expansions.append(&mut expand_code_once(&expansion, num_key_paths, dir_key_paths));
    }
    expansions = new_expansions;
    
    let tallies = remove_duplicate_tallies(expansions.iter().map(|e| tally_subsequences(e)).collect_vec());

    let mut new_expansions = vec![];
    for expansion in expansions {
        new_expansions.append(&mut expand_code_once(&expansion, dir_key_paths, dir_key_paths));
    }
    expansions = new_expansions;

    let mut new_expansions = vec![];
    for expansion in expansions {
        new_expansions.append(&mut expand_code_once(&expansion, dir_key_paths, dir_key_paths));
    }
    expansions = new_expansions;

    expansions.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap().clone()
}

fn simple_expand_len(code: &[u8], key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>) -> usize {
    let code = prepend_a(code).collect_vec();
    code.iter().tuple_windows::<(_, _)>().flat_map(|(p1, p2)| key_paths[&(*p1, *p2)].first().unwrap()).count()
}

fn expand_code_once(code: &[u8], key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>, next_key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>) -> Vec<Vec<u8>> {
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

    let min = expansions.iter().map(|a| simple_expand_len(a, next_key_paths)).min().unwrap();
    expansions.retain(|a| simple_expand_len(a, next_key_paths) == min);

    expansions
}

fn tally_code_expansions(code: &[u8], key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>) -> Vec<BTreeMap<Vec<u8>, usize>> {
    let expansions = expand_code_once(code, key_paths, key_paths);
    remove_duplicate_tallies(expansions.iter().map(|e| tally_subsequences(e)).collect_vec())
}

fn expand_tallies(tallies: &BTreeMap<Vec<u8>, usize>, key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>) -> Vec<BTreeMap<Vec<u8>, usize>> {
    for (code, count) in tallies {
        let tally_expansions = tally_code_expansions(code, key_paths);
        
    }
    todo!()
}

fn expand_all_tallies(all_tallies: &Vec<BTreeMap<Vec<u8>, usize>>, key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>) -> Vec<BTreeMap<Vec<u8>, usize>> {
    for tallies in all_tallies {
        for (code, count) in tallies {
            
        }
    }
    
    todo!()
}

fn prepend_a(button_presses: &[u8]) -> impl Iterator<Item=u8> + use<'_> {
    iter::once(b'A').chain(button_presses.iter().copied())
}

// fn best_expansion(button_presses: &[u8], previous_level_costs: ) {
//     let button_presses = prepend_a(button_presses);
//
// }

fn code_complexity(
    code: &[u8],
    num_key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>,
    dir_key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>,
) -> u64 {
    let expanded_code = expand_code(code, num_key_paths, dir_key_paths);
    let numeric_part: u64 = std::str::from_utf8(&code[0..3]).unwrap().parse().unwrap();
    // println!(
    //     "{}: {}: {} * {}",
    //     std::str::from_utf8(code).unwrap(),
    //     std::str::from_utf8(&expanded_code).unwrap(),
    //     expanded_code.len(),
    //     numeric_part
    // );
    expanded_code.len() as u64 * numeric_part
}
