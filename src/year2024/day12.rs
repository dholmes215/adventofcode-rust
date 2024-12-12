//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use std::collections::HashSet;
use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};

#[derive(Default, Debug)]
struct Region {
    area: usize,
    perimeter: usize,
    side_count: usize,
    edge_segments: HashSet<(Vec2<isize>, Vec2<isize>)>,
}

const CARDINALS: [(isize, isize); 4] = [
    (-1isize, 0isize),
    (0isize, -1isize),
    (1isize, 0isize),
    (0isize, 1isize),
];

pub fn day12(input: &str) -> SolutionResult {
    let input_grid = Grid::from_u8(input.as_bytes());
    let mut region_id_grid = Grid::<i32>::new(input_grid.width(), input_grid.height());
    region_id_grid.data_mut_slice().fill(-1);

    let mut next_region_id = 0;
    for pos in input_grid.area().all_points() {
        let plant = input_grid[pos];
        if region_id_grid[pos] == -1 {
            let id = next_region_id;
            next_region_id += 1;
            flood_fill_region(
                &input_grid,
                &mut region_id_grid,
                Vec2::from_tuple(pos),
                plant,
                id,
            );
        }
    }

    let mut regions: Vec<Region> = Vec::new();
    regions.resize_with(next_region_id as usize, Region::default);
    for pos in region_id_grid.area().all_points() {
        let id = region_id_grid[pos];
        let region = regions.get_mut(id as usize).unwrap();
        region.area += 1;
        for pos2 in CARDINALS {
            let neighbor = Vec2::from_tuple(pos) + pos2;
            if !region_id_grid.area().contains(neighbor) || region_id_grid[neighbor] != id {
                region.perimeter += 1;
                region.edge_segments.insert((Vec2::from_tuple(pos), neighbor));
            }
        }
    }

    let a = regions
        .iter()
        .map(|region| region.area * region.perimeter)
        .sum::<usize>();

    // Consolidate the edge segments into sides
    for region in &mut regions {
        let mut examined_segments: HashSet<(Vec2<isize>, Vec2<isize>)> = HashSet::new();
        for edge in &region.edge_segments {
            if !examined_segments.contains(edge) {
                let out_dir = edge.1 - edge.0;
                let left = turn_left_v(out_dir);
                let right = turn_right_v(out_dir);
                // Walk left
                let mut left_neighbor = edge.0 + left;
                while region.edge_segments.contains(&(left_neighbor, left_neighbor + out_dir)) {
                    examined_segments.insert((left_neighbor, left_neighbor + out_dir));
                    left_neighbor += left;
                }
                // Walk right
                let mut right_neighbor = edge.0 + right;
                while region.edge_segments.contains(&(right_neighbor, right_neighbor + out_dir)) {
                    examined_segments.insert((right_neighbor, right_neighbor + out_dir));
                    right_neighbor += right;
                }
                region.side_count += 1;
            }
        }
    }

    let b = regions
        .iter()
        .map(|region| region.area * region.side_count)
        .sum::<usize>();

    SolutionResult::new(a, b)
}

fn flood_fill_region(
    input_grid: &Grid<u8>,
    region_id_grid: &mut Grid<i32>,
    pos: Vec2<isize>,
    plant: u8,
    id: i32,
) {
    if !input_grid.area().contains(pos) || input_grid[pos] != plant || region_id_grid[pos] == id {
        return;
    }
    region_id_grid[pos] = id;
    for cardinal in CARDINALS {
        flood_fill_region(input_grid, region_id_grid, pos + cardinal, plant, id);
    }
}

fn turn_left(card: (isize, isize)) -> (isize, isize) {
    match card {
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        _ => panic!(),
    }
}

fn turn_left_v(card: Vec2<isize>) -> Vec2<isize> {
    Vec2::from_tuple(turn_left((card.x, card.y)))
}

fn turn_right(card: (isize, isize)) -> (isize, isize) {
    match card {
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        _ => panic!(),
    }
}

fn turn_right_v(card: Vec2<isize>) -> Vec2<isize> {
    Vec2::from_tuple(turn_right((card.x, card.y)))
}
