// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::iter;
use rayon::prelude::*;

const NUM_KEYPAD_BYTES: &[u8] = b"789\n456\n123\n#0A\n";
const DIR_KEYPAD_BYTES: &[u8] = b"#^A\n<v>>\n";

pub fn day21(input: &str) -> SolutionResult {
    let door_codes = input.lines().map(|l| l.as_bytes()).collect_vec();
    let num_keypad = Grid::from_u8(NUM_KEYPAD_BYTES);
    let dir_keypad = Grid::from_u8(DIR_KEYPAD_BYTES);

    let num_key_paths = find_keypad_paths(&num_keypad);
    let dir_key_paths = find_keypad_paths(&dir_keypad);

    let a: u64 = door_codes
        .par_iter()
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

fn expand_code(
    code: &[u8],
    num_key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>,
    dir_key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>,
) -> Vec<u8> {
    let code = code.to_vec();
    let mut expansions = vec![code.to_vec()];
    
    let mut new_expansions = vec![];
    for expansion in expansions {
        new_expansions.append(&mut expand_code_once(&expansion, num_key_paths));
    }
    expansions = new_expansions;
    
    let mut new_expansions = vec![];
    for expansion in expansions {
        new_expansions.append(&mut expand_code_once(&expansion, dir_key_paths));
    }
    expansions = new_expansions;

    let mut new_expansions = vec![];
    for expansion in expansions {
        new_expansions.append(&mut expand_code_once(&expansion, dir_key_paths));
    }
    expansions = new_expansions;

    expansions.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap().clone()
}

fn expand_code_once(code: &[u8], key_paths: &HashMap<(u8, u8), Vec<Vec<u8>>>) -> Vec<Vec<u8>> {
    let code = iter::once(b'A').chain(code.iter().copied()).collect_vec();
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


    expansions
}

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
