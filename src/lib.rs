//
// Copyright (c) 2022 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use crate::aoc::SolutionResult;

pub mod aoc {
    pub struct SolutionResult{
        pub a: String,
        pub b: String,
    }
}

impl SolutionResult {
    pub fn new<T1: ToString, T2: ToString>(a: T1, b: T2) -> SolutionResult {
        SolutionResult { a: a.to_string(), b: b.to_string() }
    }
}

struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Grid{ data: Vec::new(), width, height };
        grid.data.resize_with(grid.width * grid.height, Default::default);
        grid
    }
    
    pub fn rows(&self) -> impl Iterator + '_ {
        self.data.chunks(self.width)
    }
    
    // pub fn cols(&self) -> impl Iterator + '_ {
    //     (0..self.width).map(|i| )
    // }
}