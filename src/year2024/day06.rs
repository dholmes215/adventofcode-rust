//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};

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

    print_grid(&grid);
    let start = grid
        .area()
        .all_points()
        .find(|p| grid[*p] == b'^')
        .unwrap();
    let mut pos = Vec2::new(start.0, start.1);
    let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    while grid.area().contains(pos) {
        let mut next = pos + directions[0];
        while grid.area().contains(next) && grid[next] == b'#' {
            directions.rotate_left(1);
            next = pos + directions[0];
        }
        grid[pos] = b'X';
        pos = next;

        // print_grid(&grid);
    }

    print_grid(&grid);

    let a = grid.data_slice().iter().filter(|&&c| c == b'X').count();

    SolutionResult::new(a, "")
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
