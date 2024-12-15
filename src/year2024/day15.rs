//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use itertools::Itertools;
use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};

pub fn day15(input: &str) -> SolutionResult {
    let first_move = input
        .as_bytes()
        .iter()
        .position(|&x| b"^v><".contains(&x))
        .unwrap();
    let (input_grid, moves) = input.as_bytes().split_at(first_move);
    let moves = moves.iter().copied().filter(|x| b"^v><".contains(x)).collect_vec();
    let mut grid_a = Grid::from_u8(input_grid);
    let mut grid_b = widen_grid(&grid_a);

    part1_run(&moves, &mut grid_a);
    let a = grid_a.area().all_points().filter(|&point| grid_a[point] == b'O').map(gps_coordinate).sum::<isize>();

    part2_run(&moves, &mut grid_b);
    let b = grid_b.area().all_points().filter(|&point| grid_b[point] == b'[').map(gps_coordinate).sum::<isize>();

    SolutionResult::new(a, b)
}

fn part1_run(moves: &Vec<u8>, grid: &mut Grid<u8>) {
    let mut robot = Vec2::from_tuple(
        grid.area()
            .all_points()
            .find(|&point| grid[point] == b'@')
            .unwrap(),
    );

    for m in moves {
        let step = step(*m);
        let first_step = robot + step;
        let mut empty_or_wall_pos = first_step;
        while !b".#".contains(&grid[empty_or_wall_pos]) {
            empty_or_wall_pos += step;
        }
        let found = grid[empty_or_wall_pos];
        if found != b'#' {
            // Didn't hit a wall, so move
            grid[robot] = b'.';
            grid[empty_or_wall_pos] = grid[first_step];
            robot = first_step;
            grid[robot] = b'@';
        }
    }
}

fn can_move_box_vert(grid: &Grid<u8>, box_pos: Vec2<isize>, m: u8) -> bool {
    let (l, r) = (box_pos, box_pos + (1, 0));
    if grid[l + step(m)] == b'#' || grid[r + step(m)] == b'#' {
        return false;
    }
    if grid[l + step(m)] == b'.' && grid[r + step(m)] == b'.' {
        return true;
    }
    if grid[l + step(m)] == b'[' && grid[r + step(m)] == b']' {
        return can_move_box_vert(grid, l + step(m), m);
    }
    let l_good = grid[l + step(m)] == b'.' || can_move_box_vert(grid, l + step(m) + (-1, 0), m);
    let r_good = grid[r + step(m)] == b'.' || can_move_box_vert(grid, r + step(m), m);
    l_good & r_good
}

fn can_move_box_horiz(grid: &Grid<u8>, box_pos: Vec2<isize>, m: u8) -> bool {
    let (l, r) = (box_pos, box_pos + (1, 0));
    let adjacent_tile = match m {
        b'<' => l + (-1, 0),
        b'>' => r + (1, 0),
        _ => panic!()
    };
    match grid[adjacent_tile] {
        b'#' => false,
        b'.' => true,
        b'[' => can_move_box_horiz(grid, r + (1, 0), m),
        b']' => can_move_box_horiz(grid, l + (-2, 0), m),
        _ => panic!()
    }
}

fn can_move_box(grid: &Grid<u8>, box_pos: Vec2<isize>, m: u8) -> bool {
    match m {
        b'<'|b'>' => can_move_box_horiz(grid, box_pos, m),
        b'^'|b'v' => can_move_box_vert(grid, box_pos, m),
        _ => panic!(),
    }
}

fn move_box_vert(grid: &mut Grid<u8>, box_pos: Vec2<isize>, m: u8) {
    let (l, r) = (box_pos, box_pos + (1, 0));
    
    // Move boxes blocking us
    if grid[l + step(m)] == b'[' && grid[r + step(m)] == b']' {
        move_box_vert(grid, l + step(m), m);
    } else {
        if grid[l + step(m)] == b']' {
            move_box_vert(grid, l + step(m) + (-1, 0), m);
        }
        if grid[r + step(m)] == b'[' {
            move_box_vert(grid, r + step(m), m);
        }
    }
    
    // Now move our own box
    grid[l + step(m)] = b'[';
    grid[r + step(m)] = b']';
    grid[l] = b'.';
    grid[r] = b'.';
}

fn move_box_horiz(grid: &mut Grid<u8>, box_pos: Vec2<isize>, m: u8) {
    let (l, r) = (box_pos, box_pos + (1, 0));
    
    let adjacent_tile = match m {
        b'<' => l + (-1, 0),
        b'>' => r + (1, 0),
        _ => panic!()
    };

    // Move boxes in our way
    match grid[adjacent_tile] {
        b'[' => move_box_horiz(grid, r + (1, 0), m),
        b']' => move_box_horiz(grid, l + (-2, 0), m),
        _ => {}
    }

    // Now move our own box
    match m {
        b'<' => {
            grid[adjacent_tile] = b'[';
            grid[l] = b']';
            grid[r] = b'.';
        },
        b'>' => {
            grid[adjacent_tile] = b']';
            grid[l] = b'.';
            grid[r] = b'[';
        },
        _ => panic!()
    };
}

fn try_move_box(grid: &mut Grid<u8>, box_pos: Vec2<isize>, m: u8) {
    if !can_move_box(grid, box_pos, m) {
        return;
    }

    match m {
        b'<'|b'>' => move_box_horiz(grid, box_pos, m),
        b'^'|b'v' => move_box_vert(grid, box_pos, m),
        _ => panic!(),
    }
}

fn part2_run(moves: &Vec<u8>, grid: &mut Grid<u8>) {
    let mut robot = Vec2::from_tuple(
        grid.area()
            .all_points()
            .find(|&point| grid[point] == b'@')
            .unwrap(),
    );

    for m in moves {
        let target = robot + step(*m);
        // If there's a box, try to move it
        match grid[target] {
            b']' => {
                try_move_box(grid, target + (-1, 0), *m);
            }
            b'[' => {
                try_move_box(grid, target, *m);
            },
           _ => {} // No box
        }
        // Try to move to an empty space if there is one now
        if grid[target] == b'.'{
            grid[robot] = b'.';
            robot = target;
            grid[robot] = b'@';
        }
    }
}

fn widen_grid(grid: &Grid<u8>) -> Grid<u8> {
    let mut wide_grid = Grid::new(grid.width() * 2, grid.height());

    for pos in grid.area().all_points() {
        let l = (pos.0 * 2, pos.1);
        let r = (pos.0 * 2 + 1, pos.1);
        match grid[pos] {
            b'#' => {
                wide_grid[l] = b'#';
                wide_grid[r] = b'#';
            }
            b'O' => {
                wide_grid[l] = b'[';
                wide_grid[r] = b']';
            }
            b'.' => {
                wide_grid[l] = b'.';
                wide_grid[r] = b'.';
            }
            b'@' => {
                wide_grid[l] = b'@';
                wide_grid[r] = b'.';
            }
            _ => panic!(),
        }
    }

    wide_grid
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

fn step(c: u8) -> Vec2<isize> {
    Vec2::from_tuple(match c {
        b'^' => (0, -1),
        b'v' => (0, 1),
        b'>' => (1, 0),
        b'<' => (-1, 0),
        _ => panic!(),
    })
}

fn gps_coordinate((x, y): (isize, isize)) -> isize {
    x + y * 100
}
