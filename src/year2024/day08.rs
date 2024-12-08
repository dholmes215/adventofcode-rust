//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};
use itertools::Itertools;
use std::collections::HashMap;

pub fn day08(input: &str) -> SolutionResult {
    let mut grid = Grid::from_u8(input.as_bytes());
    let mut antenna_coords: HashMap<u8, Vec<Vec2<isize>>> = HashMap::new();
    for (x, y) in grid.area().all_points() {
        let antenna = grid[(x, y)];
        if antenna.is_ascii_alphanumeric() {
            antenna_coords
                .entry(antenna)
                .or_default()
                .push(Vec2 { x, y });
        }
    }

    let mut grid_b = grid.clone();
    for (_, coords) in &antenna_coords {
        for (pos1, pos2) in coords.iter().tuple_combinations::<(_, _)>() {
            let delta = *pos1 - *pos2;
            // Place part one antinodes in the original grid
            for antinode in [*pos1 + delta, *pos2 - delta] {
                if grid.area().contains(antinode) {
                    grid[antinode] = b'#';
                }
            }

            // Place part two antinodes in a second grid
            let mut antinode_b = *pos1;
            while grid.area().contains(antinode_b - delta) {
                antinode_b -= delta;
            }
            while grid.area().contains(antinode_b) {
                grid_b[antinode_b] = b'#';
                antinode_b += delta;
            }
        }
    }

    let a = grid.data_slice().iter().filter(|&&c| c == b'#').count();
    let b = grid_b.data_slice().iter().filter(|&&c| c == b'#').count();

    SolutionResult::new(a, b)
}
