// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{Grid, Rect, SolutionResult, Vec2};
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap, VecDeque};

pub fn day20(input: &str) -> SolutionResult {
    let mut grid = Grid::from_u8(input.as_bytes());
    let start = grid_find(&grid, b'S').unwrap();
    let end = grid_find(&grid, b'E').unwrap();
    grid[start] = b'.';
    grid[end] = b'.';


    let cost_to_start = bfs(&grid, start);
    let cost_to_end = bfs(&grid, end);

    let mut area_without_walls = grid.area();
    area_without_walls.base += (1, 1);
    area_without_walls.dimensions -= (1, 1);
    let mut cheat_results = vec![];
    for pos in area_without_walls.all_points().map(Vec2::from_tuple) {
        if grid[pos] == b'#' {
            if grid[pos + (1, 0)] == b'.' && grid[pos + (-1, 0)] == b'.' {
                cheat_results.push((cost_to_end[pos + (1, 0)].abs_diff(cost_to_end[pos + (-1, 0)])) - 2);
            }
            if grid[pos + (0, 1)] == b'.' && grid[pos + (0, -1)] == b'.' {
                cheat_results.push((cost_to_end[pos + (0, 1)].abs_diff(cost_to_end[pos + (0, -1)])) - 2);
            }
        }
    }
    cheat_results.sort();
    let a = cheat_results.iter().filter(|i| **i >= 100).count();

    
    let part2_candidates = part2_cheat_candidates();

    let mut cheat_results2 = vec![];
    for pos1 in grid.area().all_points().map(Vec2::from_tuple) {
        if grid[pos1] == b'.' {
            for cheat in &part2_candidates {
                let pos2 = pos1 + *cheat;
                let cheat_cost = (cheat.x.abs() + cheat.y.abs()) as usize;
                if grid.area().contains(pos2) && grid[pos2] == b'.' {
                    let both_reachable = cost_to_start[pos1] != -1 && cost_to_end[pos2] != -1;
                    if both_reachable {
                        let original_cost = cost_to_end[start];
                        let cost_with_cheat = cost_to_start[pos1] + cost_to_end[pos2] + cheat_cost as isize;
                        let cheat_savings = original_cost - cost_with_cheat;

                        cheat_results2.push(cheat_savings);
                    }
                }
            }
        }
    }
    cheat_results2.sort();

    let b = cheat_results2.iter().filter(|i| **i >= 100).count();

    SolutionResult::new(a, b)
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

fn bfs(
    grid: &Grid<u8>,
    start: Vec2<isize>,
) -> Grid<isize> {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut explored = Grid::<bool>::new(grid.width(), grid.height());
    explored[start] = true;
    let mut cost = Grid::<isize>::new(grid.width(), grid.height());
    cost.data_mut_slice().fill(-1);
    cost[start] = 0;

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        let v_cost = cost[v];
        for neighbor in neighbors(grid, &v) {
            if !explored[neighbor] {
                explored[neighbor] = true;
                cost[neighbor] = v_cost + 1;
                queue.push_back(neighbor);
            }
        }
    }
    cost
}

fn grid_find(grid: &Grid<u8>, target: u8) -> Option<Vec2<isize>> {
    grid.area()
        .all_points()
        .find(|&point| grid[point] == target)
        .map(Vec2::from_tuple)
}

fn part2_cheat_candidates() -> Vec<Vec2<isize>> {
    Rect {
        base: Vec2::new(-21isize, -21),
        dimensions: Vec2::new(21, 21),
    }
    .all_points()
    .map(Vec2::from_tuple)
    .filter(|p| p.x.abs() + p.y.abs() <= 20)
    .collect_vec()
}
