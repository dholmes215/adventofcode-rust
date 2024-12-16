//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use std::cmp::Reverse;
use crate::year2024::day16::Direction::{East, North, South, West};
use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};
use std::collections::{BinaryHeap, HashMap};

type State = (Vec2<isize>, Direction);
type Cost = i64;
type Move = (State, Cost);

pub fn day16(input: &str) -> SolutionResult {
    let mut grid = Grid::from_u8(input.as_bytes());
    let start = (grid_find(&grid, b'S').unwrap(), East);
    let end_pos = grid_find(&grid, b'E').unwrap();
    let ends = Direction::all().map(|d| (end_pos, d));
    grid[start.0] = b'.';
    grid[end_pos] = b'.';

    let a_result = dijkstra(&grid, start);
    
    // for end in ends {
    //     println!("{:?}", a_result.dist[&end]);
    // }

    let a = ends.iter().map(|e| a_result.dist[e]).min().unwrap();

    SolutionResult::new(a, 0)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn step(&self) -> Vec2<isize> {
        match self {
            North => Vec2::new(0, -1),
            East => Vec2::new(1, 0),
            South => Vec2::new(0, 1),
            West => Vec2::new(-1, 0),
        }
    }

    fn left(&self) -> Direction {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn right(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn all() -> [Direction; 4] {
        [North, East, South, West]
    }
}

fn neighbors(grid: &Grid<u8>, (pos, dir): State) -> Vec<Move> {
    let mut out = vec![((pos, dir.left()), 1000), ((pos, dir.right()), 1000)];
    let forward = pos + dir.step();
    if grid.area().contains(forward) && grid[forward] != b'#' {
        out.push(((forward, dir), 1));
    }
    out
}

struct DijkstraResult {
    dist: HashMap<State, Cost>,
    prev: HashMap<State, State>,
}

impl DijkstraResult {
    fn new(source: State) -> DijkstraResult {
        let mut out = DijkstraResult {
            dist: HashMap::new(),
            prev: HashMap::new(),
        };
        out.dist.insert(source, 0);
        out
    }
}

fn dijkstra(grid: &Grid<u8>, source: State) -> DijkstraResult {
    let mut o = DijkstraResult::new(source);
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((source, 0)));

    while let Some(Reverse((u_state, cost))) = queue.pop() {
        // println!("Popped {:?}", (u_state, cost));
        let neighbors = neighbors(grid, u_state);
        // println!("  Neighbors {:?}", neighbors);
        for (v_neighbor, cost) in neighbors {
            let alt = o.dist[&u_state] + cost;
            if !o.dist.contains_key(&v_neighbor) || alt < o.dist[&v_neighbor] {
                o.prev.insert(v_neighbor, u_state);
                o.dist.insert(v_neighbor, alt);
                queue.push(Reverse((v_neighbor, alt)));
                // println!("  Pushing {:?}", (v_neighbor, alt));
            }
        }
    }

    o
}

fn grid_find(grid: &Grid<u8>, target: u8) -> Option<Vec2<isize>> {
    grid.area()
        .all_points()
        .find(|&point| grid[point] == target)
        .map(Vec2::from_tuple)
}
