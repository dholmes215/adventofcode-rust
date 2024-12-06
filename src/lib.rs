//
// Copyright (c) 2022 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use std::ops::{Index, IndexMut};
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

    // pub fn rows_mut(&mut self) -> impl Iterator + '_ {
    //     self.data.as_mut_slice().chunks(self.width)
    // }
    // 
    // // pub fn row(&mut self, row: usize) {
    // //     self.rows().nth(row).unwrap()
    // // }
    // 
    // pub fn cols_mut(&mut self) -> impl Iterator + '_ {
    //     (0..self.width).map(|i| self.data.iter_mut().skip(i).step_by(self.width))
    // }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 + self.width * index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 + self.width * index.1]
    }
}

struct RowView<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
}

impl<'a, T> Index<usize> for RowView<'a, T> {
    type Output = T;
    
    fn index(&self, col: usize) -> &Self::Output {
        &self.grid[(col, self.row)]
    }
}

struct RowViewMut<'a, T> {
    grid: &'a mut Grid<T>,
    row: usize,
}

impl<'a, T> Index<usize> for RowViewMut<'a, T> {
    type Output = T;

    fn index(&self, col: usize) -> &Self::Output {
        &self.grid[(col, self.row)]
    }
}

impl<'a, T> IndexMut<usize> for RowViewMut<'a, T> {
    fn index_mut(&mut self, col: usize) -> &mut Self::Output {
        &mut self.grid[(col, self.row)]
    }
}
