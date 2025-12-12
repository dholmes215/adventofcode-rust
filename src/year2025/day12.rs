//
// Copyright (c) 2025 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Default, Eq, PartialEq)]
struct Shape {
    data: [u8; 9],
}

struct Region {
    grid: Grid<u8>,
    shape_counts: [u32; 6],
}

impl Shape {
    pub fn new(data: &[&str; 3]) -> Self {
        let mut out: Self = Default::default();
        data.iter()
            .map(|row| row.as_bytes())
            .flatten()
            .enumerate()
            .for_each(|(i, c)| out.data[i] = *c);
        out
    }

    fn coord_to_index(x: isize, y: isize) -> usize {
        (y * 3 + x) as usize
    }

    fn index_to_coord(index: usize) -> (isize, isize) {
        let index = index as isize;
        (index % 3, index / 3)
    }

    const EDGE_COORDS_IN_ORDER: [(isize, isize); 8] = [
        (0, 0),
        (1, 0),
        (2, 0),
        (2, 1),
        (2, 2),
        (1, 2),
        (0, 2),
        (0, 1),
    ];

    pub fn rotated(&self) -> Self {
        let mut out: Self = Default::default();
        for i in 0..8 {
            let src = Self::EDGE_COORDS_IN_ORDER[i];
            let dest = Self::EDGE_COORDS_IN_ORDER[(i + 1) % 8];
            out[dest] = self[src];
        }
        out
    }

    fn print(&self) {
        println!("{}", str::from_utf8(&self.data[0..3]).unwrap());
        println!("{}", str::from_utf8(&self.data[3..6]).unwrap());
        println!("{}", str::from_utf8(&self.data[6..9]).unwrap());
    }
}

impl Index<(isize, isize)> for Shape {
    type Output = u8;
    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        &self.data[Shape::coord_to_index(x, y)]
    }
}

impl IndexMut<(isize, isize)> for Shape {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        &mut self.data[Shape::coord_to_index(x, y)]
    }
}

impl Region {
    pub fn new(line: &str) -> Self {
        let (first, second) = line.split(':').collect_tuple().unwrap();
        let (width, depth) = first
            .split('x')
            .map(|num| num.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        let mut out = Region {
            grid: Grid::new(width, depth),
            shape_counts: Default::default(),
        };
        second[1..]
            .split(' ')
            .map(|num| num.parse::<u32>().unwrap())
            .enumerate()
            .for_each(|(i, num)| out.shape_counts[i] = num);
        out
    }

    pub fn solve_part1(&self) -> Option<bool> {
        // Some basic checks before actually solving properly
        let region_area = self.grid.width() as u32 * self.grid.height() as u32;
        let min_area_needed = self
            .shape_counts
            .iter()
            .map(|n| (*n as u32) * 7)
            .sum::<u32>();
        if region_area < min_area_needed {
            return Some(false);
        }

        let three_by_three_tiles = (self.grid.width() / 3 * self.grid.height() / 3) as u32;
        let three_by_three_shapes = self.shape_counts.iter().sum::<u32>();
        if three_by_three_tiles >= three_by_three_shapes {
            return Some(true);
        }

        // Give up, because solving this problem is hard and we're all tired and lazy
        // The actual input can be solved with the above checks only, but the example input can't.
        // The example input, however, could probably be solved by actually searching the space of
        // possible tilings, but that would be a bunch of effort that would never be viable for the
        // real input because its regions are vastly too large.  
        None
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x{}: {}",
            self.grid.width(),
            self.grid.height(),
            self.shape_counts.iter().join(" ")
        )
    }
}

pub fn day12(input: &str) -> SolutionResult {
    let lines = input.lines().collect_vec();
    let shape_lines = &lines[0..30];
    let shapes = shape_lines
        .chunks(5)
        .map(|chunk| Shape::new(<&[&str; 3]>::try_from(&chunk[1..4]).unwrap()))
        .collect_vec();
    let region_lines = &lines[30..];
    let regions = region_lines.iter().copied().map(Region::new).collect_vec();

    // for region in &regions {
    //     let region_area = region.grid.width() * region.grid.height();
    //     let min_area_needed = region
    //         .shape_counts
    //         .iter()
    //         .map(|n| (*n ) * 7)
    //         .sum::<u32>();
    //     let fill_ratio = min_area_needed as f64 / region_area as f64;
    //     println!(
    //         "{}:  area: {}, minimum area needed: {}, fill ratio: {:.2}, fits: {:?}",
    //         region,
    //         region_area,
    //         min_area_needed,
    //         fill_ratio,
    //         region.solve_part1()
    //     );
    // }

    let a_solutions = regions.iter().map(Region::solve_part1).collect_vec();
    for (i, a_solution) in a_solutions.iter().enumerate() {
        println!("{}: {:?}", i, a_solution);
    }
    let a = if a_solutions.iter().any(|&x| x.is_none()) {
        "Failed to solve problem".to_string()
    } else {
        regions.iter().map(Region::solve_part1).filter(|&x| x == Some(true)).count().to_string()
    };
    let b = "";

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}
