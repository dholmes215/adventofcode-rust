//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};
use itertools::Itertools;

const CARDINALS: [(isize, isize); 4] = [
    (0isize, 1isize),
    (1isize, 0isize),
    (0isize, -1isize),
    (-1isize, 0isize),
];

pub fn valid_next_steps(grid: &Grid<u8>, pos: Vec2<isize>) -> Vec<Vec2<isize>> {
    CARDINALS
        .map(|c| pos + c)
        .into_iter()
        .filter(|p| grid.area().contains(*p) && grid[*p] == grid[pos] + 1)
        .collect()
}

pub fn reachable_ends(grid: &Grid<u8>, trailhead: Vec2<isize>) -> Vec<Vec2<isize>> {
    if grid[trailhead] == 9 {
        return vec![trailhead];
    }
    valid_next_steps(grid, trailhead)
        .iter()
        .flat_map(|p| reachable_ends(grid, *p))
        .sorted_unstable()
        .unique()
        .collect()
}

pub fn trailhead_score(grid: &Grid<u8>, trailhead: Vec2<isize>) -> usize {
    reachable_ends(grid, trailhead).len()
}

pub fn trailhead_rating(grid: &Grid<u8>, trailhead: Vec2<isize>) -> usize {
    if grid[trailhead] == 9 {
        return 1;
    }
    valid_next_steps(grid, trailhead)
        .iter()
        .map(|p| trailhead_rating(grid, *p))
        .sum()
}

pub fn day10(input: &str) -> SolutionResult {
    let mut grid = Grid::from_u8(input.as_bytes());
    for p in grid.area().all_points() {
        if (grid[p] != b'.') {
            grid[p] -= b'0';
        }
    }

    let trailheads = grid
        .area()
        .all_points()
        .filter(|p| grid[*p] == 0)
        .map(Vec2::from_tuple)
        .collect_vec();

    let a: usize = trailheads.iter().map(|p| trailhead_score(&grid, *p)).sum();
    let b: usize = trailheads.iter().map(|p| trailhead_rating(&grid, *p)).sum();

    SolutionResult::new(a, b)
}
