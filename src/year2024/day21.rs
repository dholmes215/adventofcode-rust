// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::iter;
use std::str::FromStr;

const NUM_KEYPAD_BYTES: &[u8] = b"789\n456\n123\n#0A\n";
const DIR_KEYPAD_BYTES: &[u8] = b"#^A\n<v>>\n";

pub fn day21(input: &str) -> SolutionResult {
    let door_codes = input.lines().map(|l| l.as_bytes()).collect_vec();
    let num_keypad = Grid::from_u8(NUM_KEYPAD_BYTES);
    let dir_keypad = Grid::from_u8(DIR_KEYPAD_BYTES);

    let num_key_paths = find_keypad_paths(&num_keypad);
    let dir_key_paths = find_keypad_paths(&dir_keypad);

    // for code in door_codes {
    //     let code2 = iter::once(b'A').chain(code.iter().copied());
    //     println!(
    //         "{}: {}",
    //         std::str::from_utf8(code).unwrap(),
    //         String::from_utf8(expand_code(code, &num_key_paths, &dir_key_paths)).unwrap()
    //     )
    // }

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

fn bfs(grid: &Grid<u8>, start: Vec2<isize>) -> Grid<Vec2<isize>> {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut explored = Grid::<bool>::new(grid.width(), grid.height());
    explored[start] = true;
    let mut predecessors = Grid::<Vec2<isize>>::new(grid.width(), grid.height());

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();

        for neighbor in neighbors(grid, &v) {
            if !explored[neighbor] {
                explored[neighbor] = true;
                predecessors[neighbor] = v;
                queue.push_back(neighbor);
            }
        }
    }
    predecessors
}

fn find_path(
    predecessors: &Grid<Vec2<isize>>,
    start: Vec2<isize>,
    end: Vec2<isize>,
) -> Vec<Vec2<isize>> {
    let mut out = vec![];
    let mut current = end;
    out.push(current);
    while current != start {
        current = predecessors[current];
        out.push(current);
    }
    out.reverse();
    out
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

fn find_keypad_paths(grid: &Grid<u8>) -> HashMap<(u8, u8), Vec<u8>> {
    let keys = grid
        .data_slice()
        .iter()
        .filter(|b| **b != b'#')
        .collect_vec();
    let mut paths = HashMap::<(u8, u8), Vec<u8>>::new();
    for button1 in &keys {
        let pos1 = grid_find(grid, **button1).unwrap();
        let predecessors = bfs(grid, pos1);
        for button2 in &keys {
            let pos2 = grid_find(grid, **button2).unwrap();
            paths.insert(
                (grid[pos1], grid[pos2]),
                path_to_u8(&find_path(&predecessors, pos1, pos2)),
            );
        }
    }
    paths
}

fn expand_code(
    code: &[u8],
    num_key_paths: &HashMap<(u8, u8), Vec<u8>>,
    dir_key_paths: &HashMap<(u8, u8), Vec<u8>>,
) -> Vec<u8> {
    // Start at 'A'
    let mut code = code.to_vec();
    println!("{}", std::str::from_utf8(&code).unwrap());
    code = expand_code_once(&code, num_key_paths);
    println!("{}", std::str::from_utf8(&code).unwrap());
    code = expand_code_once(&code, dir_key_paths);
    println!("{}", std::str::from_utf8(&code).unwrap());
    code = expand_code_once(&code, dir_key_paths);
    println!("{}", std::str::from_utf8(&code).unwrap());
    code
}

fn expand_code_once(code: &[u8], key_paths: &HashMap<(u8, u8), Vec<u8>>) -> Vec<u8> {
    let code = iter::once(b'A').chain(code.iter().copied()).collect_vec();
    code.iter()
        .copied()
        .tuple_windows::<(_, _)>()
        .flat_map(|(a, b)| key_paths[&(a, b)].clone())
        .collect_vec()
}

fn code_complexity(
    code: &[u8],
    num_key_paths: &HashMap<(u8, u8), Vec<u8>>,
    dir_key_paths: &HashMap<(u8, u8), Vec<u8>>,
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
