//
// Copyright (c) 2025 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;

pub fn day04(input: &str) -> SolutionResult {
    let mut grid = Grid::from_u8(input.as_bytes());

    let a = grid
        .area()
        .all_points()
        .filter(|p| check_position(&grid, Vec2::from_tuple(*p)))
        .count();

    let rolls_before = grid.data_slice().iter().filter(|&&t| t == b'@').count();
    remove_all_possible_rolls(&mut grid);
    let rolls_after = grid.data_slice().iter().filter(|&&t| t == b'@').count();

    let b = rolls_before - rolls_after;

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}

const NEIGHBOR_DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
];

fn neighbor_tiles<'a>(grid: &'a Grid<u8>, pos: Vec2<isize>) -> impl Iterator<Item = u8> + 'a {
    NEIGHBOR_DIRECTIONS
        .into_iter()
        .map(move |d| pos + d)
        .filter(|&p| grid.area().contains(p))
        .map(|p| grid[p])
}

fn count_neighbor_rolls(grid: &Grid<u8>, pos: Vec2<isize>) -> usize {
    neighbor_tiles(grid, pos).filter(|&t| t == b'@').count()
}

fn check_position(grid: &Grid<u8>, p: Vec2<isize>) -> bool {
    grid[p] == b'@' && count_neighbor_rolls(grid, p) < 4
}

fn remove_all_possible_rolls(grid: &mut Grid<u8>) {
    let mut removed = true;
    while removed {
        removed = false;
        for p in grid.area().all_points() {
            if check_position(grid, Vec2::from_tuple(p)) {
                grid[p] = b'x';
                removed = true;
            }
        }
    }
}