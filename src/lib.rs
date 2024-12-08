//
// Copyright (c) 2022 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//
#![feature(step_trait)]
use crate::aoc::SolutionResult;

pub mod aoc {
    use std::fmt::Display;
    use itertools::{Itertools, Product};
    use std::ops;
    use std::ops::{Add, Index, IndexMut, Range};

    pub struct SolutionResult {
        pub a: String,
        pub b: String,
    }

    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Vec2<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Vec2<T> {
        pub fn new(x: T, y: T) -> Vec2<T> {
            Vec2 { x, y }
        }
        pub fn from_tuple(t: (T, T)) -> Vec2<T> {
            Vec2::new(t.0, t.1)
        }
    }
    
    impl<T: Display> Display for Vec2<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl<T: ops::Add<Output = T>> ops::Add<Vec2<T>> for Vec2<T> {
        type Output = Vec2<T>;

        fn add(self, rhs: Vec2<T>) -> Self::Output {
            Vec2::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl<T: ops::Add<Output = T>> ops::Add<(T, T)> for Vec2<T> {
        type Output = Vec2<T>;

        fn add(self, rhs: (T, T)) -> Self::Output {
            Vec2::new(self.x + rhs.0, self.y + rhs.1)
        }
    }

    impl<T: ops::AddAssign> ops::AddAssign<Vec2<T>> for Vec2<T> {
        fn add_assign(&mut self, rhs: Vec2<T>) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl<T: ops::AddAssign> ops::AddAssign<(T, T)> for Vec2<T> {
        fn add_assign(&mut self, rhs: (T, T)) {
            self.x += rhs.0;
            self.y += rhs.1;
        }
    }

    impl<T: ops::Sub<Output = T>> ops::Sub<Vec2<T>> for Vec2<T> {
        type Output = Vec2<T>;

        fn sub(self, rhs: Vec2<T>) -> Self::Output {
            Vec2::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl<T: ops::Sub<Output = T>> ops::Sub<(T, T)> for Vec2<T> {
        type Output = Vec2<T>;

        fn sub(self, rhs: (T, T)) -> Self::Output {
            Vec2::new(self.x - rhs.0, self.y - rhs.1)
        }
    }

    impl<T: ops::SubAssign> ops::SubAssign<Vec2<T>> for Vec2<T> {
        fn sub_assign(&mut self, rhs: Vec2<T>) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }

    impl<T: ops::SubAssign> ops::SubAssign<(T, T)> for Vec2<T> {
        fn sub_assign(&mut self, rhs: (T, T)) {
            self.x -= rhs.0;
            self.y -= rhs.1;
        }
    }

    impl<T: Clone + ops::Mul<Output = T>> ops::Mul<&T> for Vec2<T> {
        type Output = Vec2<T>;

        fn mul(self, rhs: &T) -> Self::Output {
            Vec2::new(self.x * rhs.clone(), self.y * rhs.clone())
        }
    }

    impl<T: Clone + ops::MulAssign> ops::AddAssign<&T> for Vec2<T> {
        fn add_assign(&mut self, rhs: &T) {
            self.x *= rhs.clone();
            self.y *= rhs.clone();
        }
    }

    pub struct Rect<T> {
        pub base: Vec2<T>,
        pub dimensions: Vec2<T>,
    }

    impl<T: std::cmp::PartialOrd> Rect<T> {
        pub fn contains(&self, point: Vec2<T>) -> bool {
            self.base.x <= point.x
                && point.x < self.dimensions.x
                && self.base.y <= point.y
                && point.y < self.dimensions.y
        }
    }

    impl<T: Copy + std::iter::Step> Rect<T> {
        pub fn all_points(&self) -> Product<Range<T>, Range<T>> {
            (self.base.y..self.dimensions.y).cartesian_product(self.base.x..self.dimensions.x)
        }
    }

    fn is_crlf_byte(c: &u8) -> bool {
        *c == b'\r' || *c == b'\n'
    }

    #[derive(Clone)]
    pub struct Grid<T> {
        data: Vec<T>,
        width: isize,
        height: isize,
    }

    impl Grid<u8> {
        pub fn from_u8(input: &[u8]) -> Grid<u8> {
            let width = input.iter().position(is_crlf_byte).unwrap();
            let chunk_width = input[width..].iter().position(|c| !is_crlf_byte(c)).unwrap() + width;
            let height = (input.len() / chunk_width);
            let mut grid = Grid::<u8>::new(width as isize, height as isize);
            let input_iter = input
                .trim_ascii()
                .iter()
                .filter(|c| **c != b'\r' && **c != b'\n');
            grid.data_mut_slice().iter_mut().zip(input_iter).for_each(|(to, from)| *to = *from);
            grid
        }
    }
    
    impl<T: Default> Grid<T> {
        pub fn new(width: isize, height: isize) -> Self {
            let mut grid = Grid {
                data: Vec::new(),
                width,
                height,
            };
            grid.data
                .resize_with((grid.width * grid.height) as usize, Default::default);
            grid
        }

        pub fn width(&self) -> isize {
            self.width
        }
        pub fn height(&self) -> isize {
            self.height
        }

        pub fn area(&self) -> Rect<isize> {
            Rect {
                base: Vec2::new(0, 0),
                dimensions: Vec2::new(self.width, self.height),
            }
        }

        pub fn data_slice(&self) -> &[T] {
            self.data.as_slice()
        }

        pub fn data_mut_slice(&mut self) -> &mut [T] {
            self.data.as_mut_slice()
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

    impl<T> Index<(isize, isize)> for Grid<T> {
        type Output = T;

        fn index(&self, index: (isize, isize)) -> &Self::Output {
            &self.data[(index.1 * self.height + index.0) as usize]
        }
    }

    impl<T> IndexMut<(isize, isize)> for Grid<T> {
        fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
            &mut self.data[(index.1 * self.height + index.0) as usize]
        }
    }

    impl<T> Index<Vec2<isize>> for Grid<T> {
        type Output = T;

        fn index(&self, index: Vec2<isize>) -> &Self::Output {
            &self.data[(index.y * self.height + index.x) as usize]
        }
    }

    impl<T> IndexMut<Vec2<isize>> for Grid<T> {
        fn index_mut(&mut self, index: Vec2<isize>) -> &mut Self::Output {
            &mut self.data[(index.y * self.height + index.x) as usize]
        }
    }

    struct RowView<'a, T> {
        grid: &'a Grid<T>,
        row: isize,
    }

    impl<'a, T> Index<isize> for RowView<'a, T> {
        type Output = T;

        fn index(&self, col: isize) -> &Self::Output {
            &self.grid[(col, self.row)]
        }
    }

    struct RowViewMut<'a, T> {
        grid: &'a mut Grid<T>,
        row: isize,
    }

    impl<'a, T> Index<isize> for RowViewMut<'a, T> {
        type Output = T;

        fn index(&self, col: isize) -> &Self::Output {
            &self.grid[(col, self.row)]
        }
    }

    impl<'a, T> IndexMut<isize> for RowViewMut<'a, T> {
        fn index_mut(&mut self, col: isize) -> &mut Self::Output {
            &mut self.grid[(col, self.row)]
        }
    }
}

impl SolutionResult {
    pub fn new<T1: ToString, T2: ToString>(a: T1, b: T2) -> SolutionResult {
        SolutionResult {
            a: a.to_string(),
            b: b.to_string(),
        }
    }
}
