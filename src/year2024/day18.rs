// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::VecDeque;

pub fn day18(input: &str) -> SolutionResult {
    let coordinates = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<isize>().unwrap())
        .tuples::<(_, _)>()
        .collect_vec();

    let example = coordinates.len() < 1000;
    let grid_size: Vec2<isize> = match example {
        true => Vec2::new(7, 7),
        false => Vec2::new(71, 71),
    };
    let iter_count = match example {
        true => 12,
        false => 1024,
    };

    let grid = prepare_grid(&coordinates, grid_size, iter_count);

    let a = find_path(&grid).unwrap().len() - 1;

    // Brute-force part 2 with part 1 solution
    let b = coordinates
        .par_iter()
        .enumerate()
        .skip(iter_count)
        .find_first(|&(i, _)| part2_test(i, &coordinates))
        .unwrap()
        .1;

    SolutionResult::new(a, format!("{},{}", b.0, b.1))
}

fn part2_test(i: usize, coordinates: &[(isize, isize)]) -> bool {
    let example = coordinates.len() < 1000;
    let grid_size: Vec2<isize> = match example {
        true => Vec2::new(7, 7),
        false => Vec2::new(71, 71),
    };

    let grid = prepare_grid(coordinates, grid_size, i + 1);
    find_path(&grid).is_none()
}

fn prepare_grid(
    coordinates: &[(isize, isize)],
    grid_size: Vec2<isize>,
    iter_count: usize,
) -> Grid<u8> {
    let mut grid = Grid::<u8>::new(grid_size.x, grid_size.y);
    grid.data_mut_slice().fill(b'.');
    for pos in &coordinates[..iter_count] {
        grid[*pos] = b'#';
    }
    grid
}

const CARDINALS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn neighbors<'a>(
    grid: &'a Grid<u8>,
    pos: &'a Vec2<isize>,
) -> impl Iterator<Item = Vec2<isize>> + 'a {
    CARDINALS
        .into_iter()
        .map(|c| *pos + c)
        .filter(|c| grid.area().contains(*c) && grid[*c] == b'.')
}

fn bfs(grid: &Grid<u8>) -> Option<Grid<Vec2<isize>>> {
    let start = grid.area().base;
    let end = grid.area().dimensions - (1, 1);

    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut explored = Grid::<bool>::new(grid.width(), grid.height());
    explored[start] = true;
    let mut predecessors = Grid::<Vec2<isize>>::new(grid.width(), grid.height());

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        if v == end {
            return Some(predecessors);
        } else {
            for neighbor in neighbors(grid, &v) {
                if !explored[neighbor] {
                    explored[neighbor] = true;
                    predecessors[neighbor] = v;
                    queue.push_back(neighbor);
                }
            }
        }
    }
    None
}

fn find_path(grid: &Grid<u8>) -> Option<Vec<Vec2<isize>>> {
    let start = grid.area().base;
    let end = grid.area().dimensions - (1, 1);
    match bfs(grid) {
        None => None,
        Some(predecessors) => {
            let mut out = vec![];
            let mut current = end;
            out.push(current);
            while current != start {
                current = predecessors[current];
                out.push(current);
            }
            out.reverse();
            Some(out)
        }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid<u8>) {
    for line in grid
        .data_slice()
        .chunks_exact(grid.width() as usize)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
    {
        println!("{}", line);
    }
    println!();
}
