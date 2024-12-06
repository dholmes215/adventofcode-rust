//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

pub fn day06(input: &str) -> SolutionResult {
    let width = input.lines().next().unwrap().len() as isize;
    let height = input.trim().lines().count() as isize;
    let mut grid = Grid::<u8>::new(width, height);
    grid.data_mut_slice()
        .iter_mut()
        .zip(
            input
                .trim()
                .as_bytes()
                .iter()
                .filter(|c| **c != b'\r' && **c != b'\n'),
        )
        .for_each(|(to, from)| *to = *from);

    // print_grid(&grid);

    let (grid_a, _) = simulate_guard(&grid, None);

    let a = grid_a.data_slice().iter().filter(|&&c| c == b'X').count();

    let b = grid_a
        .area()
        .all_points()
        .filter(|p| grid_a[*p] == b'X')
        .collect_vec()
        .par_iter()
        .filter(|(x, y)| simulate_guard(&grid, Some(Vec2::new(*x, *y))).1)
        .count();

    SolutionResult::new(a, b)
}

fn simulate_guard(grid: &Grid<u8>, obstacle: Option<Vec2<isize>>) -> (Grid<u8>, bool) {
    let mut grid = grid.clone();
    let start = grid.area().all_points().find(|p| grid[*p] == b'^').unwrap();
    let mut pos = Vec2::new(start.0, start.1);

    if let Some(obstacle) = obstacle {
        if pos != obstacle {
            grid[obstacle] = b'#';
        }
    }

    let mut directions = [
        Vec2::new(0, -1),
        Vec2::new(1, 0),
        Vec2::new(0, 1),
        Vec2::new(-1, 0),
    ];

    // (position, direction) pairs
    let mut visited: HashSet<(Vec2<isize>, Vec2<isize>)> = HashSet::new();
    visited.insert((pos, Vec2::new(0, -1)));

    let mut looped = false;
    while grid.area().contains(pos) && !looped {
        let mut next = pos + directions[0];
        while grid.area().contains(next) && grid[next] == b'#' {
            directions.rotate_left(1);
            next = pos + directions[0];
        }
        grid[pos] = b'X';
        pos = next;

        if visited.contains(&(pos, directions[0])) {
            looped = true;
        } else {
            visited.insert((pos, directions[0]));
        }
        // print_grid(&grid);
    }

    // print_grid(&grid);

    (grid, looped)
}

fn print_grid(grid: &Grid<u8>) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            print!("{}", grid[(x, y)] as char);
        }
        println!();
    }
    println!();
}
