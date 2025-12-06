//
// Copyright (c) 2025 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::*;
use itertools::Itertools;

pub fn day06(input: &str) -> SolutionResult {
    let lines = input.lines().collect_vec();
    let input_width = lines.iter().map(|l| l.len()).max().unwrap();
    let input_height = lines.len();

    let op_chars_by_index = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != ' ')
        .collect_vec();

    let ops = op_chars_by_index.iter().map(|(_, c)| *c).collect_vec();

    let problem_column_fields = op_chars_by_index
        .iter()
        .map(|(i, _)| *i)
        .chain([input_width + 1])
        .tuple_windows::<(_, _)>()
        .map(|(l, r)| (l, r - l - 1))
        .collect_vec();

    let problem_count = problem_column_fields.len();

    let mut grid = Grid::<char>::new((problem_count * 4) as isize, (input_height - 1) as isize);
    for (x, y) in grid.area().all_points() {
        grid[(x, y)] = ' ';
    }

    for (y, line) in lines.iter().dropping_back(1).enumerate() {
        for (i, (input_x, width)) in problem_column_fields.iter().enumerate() {
            let input_x_end = std::cmp::min(*input_x + *width, line.len());
            let copy_from = &line[*input_x..input_x_end];
            let copy_to_x = (i * 4) as isize;
            for (i2, c) in copy_from.chars().enumerate() {
                grid[(copy_to_x + i2 as isize, y as isize)] = c;
            }
        }
    }

    let mut a = 0;
    let mut b = 0;

    for problem_index in 0..problem_count {
        let x_range = (problem_index * 4) as isize..((problem_index + 1) * 4) as isize;
        let y_range = 0..(input_height - 1) as isize;

        // Human numbers
        let human_nums = y_range
            .clone()
            .map(|y| {
                let start = ((y * grid.width()) as usize + (problem_index * 4) as usize);
                grid.data_slice()[start..start + 4]
                    .into_iter()
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()
            }).flatten()
            .collect_vec();

        let solve_problems = |nums: &Vec<u64>| match ops[problem_index] {
            '+' => nums.iter().sum::<u64>(),
            '*' => nums.iter().product::<u64>(),
            _ => panic!("Unknown op: {}", ops[problem_index]),
        };
        a += solve_problems(&human_nums);

        // Cephalopod numbers
        let cephalopod_nums = x_range
            .clone()
            .map(|place| {
                y_range
                    .clone()
                    .map(|y| grid[Vec2::from_tuple((place as isize, y))])
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()
            })
            .flatten()
            .collect_vec();

        b += solve_problems(&cephalopod_nums);
    }

    SolutionResult {
        a: a.to_string(),
        b: b.to_string(),
    }
}
