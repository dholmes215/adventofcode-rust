//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use std::cmp::Reverse;
use crate::year2024::day16::Direction::{East, North, South, West};
use adventofcode_rust::aoc::{Grid, SolutionResult, Vec2};
use std::collections::{BinaryHeap, HashMap, HashSet};
use itertools::Itertools;

type State = (Vec2<isize>, Direction);
type Cost = i64;
type Move = (State, Cost);

pub fn day16(input: &str) -> SolutionResult {
    let mut grid = Grid::from_u8(input.as_bytes());
    let start = (grid_find(&grid, b'S').unwrap(), East);
    let end_pos = grid_find(&grid, b'E').unwrap();
    
    // Because there can be multiple paths with the same cost that end facing different directions,
    // there's more than one valid end "state" even if there's only one end position
    let ends = Direction::all().map(|d| (end_pos, d));
    grid[start.0] = b'.';
    grid[end_pos] = b'.';

    let a_result = dijkstra(&grid, start);
    let a = ends.iter().map(|e| a_result.dist[e]).min().unwrap();
    
    let best_ends = ends.iter().filter(|e| a_result.dist[e] == a).collect_vec();
    
    // Walk all of the best paths we've found and mark them on the map.
    let mut path_states = HashSet::<State>::new();
    for end in best_ends {
        path_states.insert(*end);
    }
    while !path_states.is_empty() {
        let state = *path_states.iter().next().unwrap();
        path_states.remove(&state);
        if let Some(predecessors) = &a_result.prev.get(&state){
            for state in *predecessors {
                path_states.insert(*state);
            }
        }
        grid[state.0] = b'O';
    }
    let b = grid.data_slice().iter().filter(|&&b| b == b'O').count();

    SolutionResult::new(a, b)
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
    prev: HashMap<State, HashSet<State>>,
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

// This is a version of Dijkstra's algorithm with several properties:
// - Uses a priority queue
// - Does not pre-initialize all vertexes and instead discovers them as it explores
// - Doesn't just replace predecessor when it finds a better one; also records multiple predecessors
//   if it finds one _equal_ to the current best, so we can discover more than one "best" path
//   between two pairs.
fn dijkstra(grid: &Grid<u8>, source: State) -> DijkstraResult {
    let mut o = DijkstraResult::new(source);
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((source, 0)));

    while let Some(Reverse((u_state, _))) = queue.pop() {
        let neighbors = neighbors(grid, u_state);
        for (v_neighbor, cost) in neighbors {
            let alt = o.dist[&u_state] + cost;
            if !o.dist.contains_key(&v_neighbor) || alt < o.dist[&v_neighbor] {
                o.prev.insert(v_neighbor, {
                    let mut set = HashSet::new();
                    set.insert(u_state);
                    set
                });
                o.dist.insert(v_neighbor, alt);
                queue.push(Reverse((v_neighbor, alt)));
            } else if o.dist.contains_key(&v_neighbor) && alt == o.dist[&v_neighbor] {
                o.prev.get_mut(&v_neighbor).unwrap().insert(u_state);
                o.dist.insert(v_neighbor, alt);
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