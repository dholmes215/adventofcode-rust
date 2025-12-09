//
// Copyright (c) 2025 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;
use itertools::Itertools;
use std::collections::VecDeque;

pub fn day09(input: &str) -> SolutionResult {
    let points: Vec<Vec2<i64>> = input
        .lines()
        .map(|line| {
            Vec2::from_tuple(
                line.split(',')
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect_tuple::<(_, _)>()
                    .unwrap(),
            )
        })
        .collect_vec();

    let mut pairs = points
        .iter()
        .cloned()
        .cartesian_product(points.iter().cloned())
        .filter(|&(a, b)| a < b)
        .collect_vec();

    pairs.sort_by_key(rect_area);
    pairs.reverse();

    let a = pairs.iter().map(rect_area).max().unwrap();

    // The non-example input's dimensions are far too large to construct an entire actual grid,
    // so instead we'll create a "compressed" grid where the points correspond to large rectangles
    // of the original space.
    //
    // The compressed grid has a column for every x coordinate that has a point, and also a column
    //  for every interval of x coordinates _without_ a point, and likewise for rows and y
    //  coordinates.

    // Determine x and y interval lists.
    let mut input_x_points = points.iter().cloned().map(|v| v.x).collect_vec();
    input_x_points.sort_unstable();
    input_x_points.dedup();
    let mut input_y_points = points.iter().cloned().map(|(v)| v.y).collect_vec();
    input_y_points.sort_unstable();
    input_y_points.dedup();

    let x_intervals = generate_interval_list(&input_x_points);
    let y_intervals = generate_interval_list(&input_y_points);

    let compress_point = |Vec2 { x, y }: Vec2<i64>| Vec2 {
        x: x_intervals.partition_point(|&(x2, _)| x2 < x) as isize,
        y: y_intervals.partition_point(|&(_, y2)| y2 < y) as isize,
    };

    // Create compressed grid.
    let mut compressed_grid: Grid<char> =
        Grid::new(x_intervals.len() as isize, y_intervals.len() as isize);
    compressed_grid.data_mut_slice().fill(' ');

    for &point in &points {
        compressed_grid[compress_point(point)] = '#';
    }

    // Connect lines.
    let first = points[0];
    let connections = points
        .iter()
        .cloned()
        .chain([first])
        .map(compress_point)
        .tuple_windows::<(_, _)>()
        .collect_vec();
    for (from, to) in connections.iter().copied() {
        let mut step = to - from;
        step.x = step.x.signum();
        step.y = step.y.signum();
        let mut next = from + step;
        while next != to {
            if compressed_grid[next] != '#' {
                compressed_grid[next] = 'X';
            }
            next += step;
        }
    }

    // Flood fill exterior.
    let mut flood_fill_queue = VecDeque::new();
    let first = Vec2 {
        x: 0isize,
        y: 0isize,
    };
    const COLOR_TO_FILL: char = ' ';
    const TEMP_COLOR: char = 'T';
    const NEIGHBORS: [Vec2<isize>; 4] = [
        Vec2 { x: 0, y: 1 },
        Vec2 { x: 1, y: 0 },
        Vec2 { x: 0, y: -1 },
        Vec2 { x: -1, y: 0 },
    ];
    let grid_area = compressed_grid.area();
    flood_fill_queue.push_back(first);
    while let Some(pos) = flood_fill_queue.pop_front() {
        compressed_grid[pos] = '.';
        for next in NEIGHBORS
            .iter()
            .map(|&n| pos + n)
            .filter(|&pos| grid_area.contains(pos))
        {
            if compressed_grid[next] == COLOR_TO_FILL {
                compressed_grid[next] = TEMP_COLOR;
                flood_fill_queue.push_back(next);
            }
        }
    }

    compressed_grid
        .data_mut_slice()
        .iter_mut()
        .for_each(|&mut ref mut c| {
            if *c == ' ' {
                *c = 'O'
            }
        });

    // Print grid.
    // const PRINT_MAX_WIDTH: isize = 200;
    // const PRINT_MAX_HEIGHT: isize = 500;
    // for y in 0isize..(compressed_grid.height().min(PRINT_MAX_HEIGHT)) {
    //     for x in 0isize..compressed_grid.width().min(PRINT_MAX_WIDTH) {
    //         print!("{}", compressed_grid[(x, y)]);
    //     }
    //     println!();
    // }

    // Filter the rectangle list we already found by whether or not all the rectangle contents are
    // red and green only.
    let compress_pair = |(l, r)| (compress_point(l), compress_point(r));
    let rect_from_compressed_pair = |(p1, p2): (Vec2<isize>, Vec2<isize>)| {
        let base: Vec2<isize> = Vec2::from_tuple((p1.x.min(p2.x), p1.y.min(p2.y)));
        let rect: Rect<isize> = Rect {
            base,
            dimensions: Vec2::from_tuple((p1.x.max(p2.x), p1.y.max(p2.y))) - base + (1, 1),
        };
        rect
    };
    let is_valid_rectangle =
        |rect: &Rect<isize>| rect.all_points().all(|p| compressed_grid[p] != '.');

    let b = rect_area(
        &pairs
            .iter()
            .cloned()
            .filter(|p| is_valid_rectangle(&rect_from_compressed_pair(compress_pair(*p))))
            .next()
            .unwrap(),
    );

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}

fn rect_area((a, b): &(Vec2<i64>, Vec2<i64>)) -> i64 {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

fn generate_interval_list(input_points: &Vec<i64>) -> Vec<(i64, i64)> {
    // Intervals are all (inclusive, exclusive) pairs.
    let mut intervals: Vec<(i64, i64)> = vec![(0, input_points[0])];
    for (a, b) in input_points.iter().cloned().tuple_windows::<(_, _)>() {
        intervals.push((a, a + 1));
        if a != b {
            intervals.push((a + 1, b));
        }
    }

    // Also add the last point, since the last tuple window wouldn't cover it.
    let last = *input_points.last().unwrap();

    intervals.push((last, last + 1));
    // Create an extra interval for space past the last point so the rectangles don't come right up
    // against the edge.
    intervals.push((last + 1, last + 2));
    intervals
}
