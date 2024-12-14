//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::{Grid, Rect, SolutionResult, Vec2};
use itertools::Itertools;
use std::hash::{DefaultHasher, Hash, Hasher};

type FourNums = (i64, i64, i64, i64);

pub fn day14(input: &str) -> SolutionResult {
    let robots = input
        .split(|c: char| !c.is_ascii_digit() && (c != '-'))
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .tuples::<FourNums>()
        .map(|n| parse_robot(&n))
        .collect_vec();

    let grid_dimensions = if robots.len() == 12 {
        Vec2::new(11, 7)
    } else {
        Vec2::new(101, 103)
    };

    let mut robots_a = robots.clone();

    let seconds = 100i64;
    for robot in robots_a.iter_mut() {
        robot.pos = wrap_pos(robot.pos + (robot.vel * &(seconds)), grid_dimensions);
    }

    let quadrant_counts = count_quadrants(grid_dimensions, &robots_a);
    let a = quadrant_counts.iter().product::<i64>();

    let mut robots_b = robots.clone();
    let start_hash = hash_robots(&robots_b);

    let mut i=0;
    let mut most_in_center = 0;
    let mut b = 0;
    loop {
        for robot in robots_b.iter_mut() {
            robot.pos = wrap_pos(robot.pos + robot.vel, grid_dimensions);
        }
        i += 1;

        // The arrangement of robots we're looking for is a Christmas tree in a square in the middle
        // of the room.  I don't know how much variation there is between inputs, but assuming they
        // all follow this general pattern, there will be more robots in the center at this time
        // than at any other time, so count robots in the center and identify the time where there
        // are most there.
        let center_square = Rect {
            base: Vec2::new(grid_dimensions.x / 4, grid_dimensions.y / 4),
            dimensions: Vec2::new(grid_dimensions.x / 2, grid_dimensions.y / 2),
        };

        let mut center_count = 0;
        for robot in robots_b.iter() {
            if center_square.contains(robot.pos) {
                center_count += 1;
            }
        }
        if most_in_center < center_count {
            most_in_center = center_count;
            b = i;
        }
        
        if start_hash == hash_robots(&robots_b) {
            // The arrangement of robots will eventually loop, so we can stop once it does
            break;
        }
    }

    SolutionResult::new(a, b)
}

fn hash_robots(robots_b: &Vec<Robot>) -> u64 {
    let mut s = DefaultHasher::new();
    robots_b.hash(&mut s);
    s.finish()
}

fn count_quadrants(grid_dimensions: Vec2<i64>, robots: &[Robot]) -> [i64; 4] {
    let mut quadrant_counts = [0, 0, 0, 0];
    for robot in robots.iter() {
        if robot.pos.x < grid_dimensions.x / 2 {
            if robot.pos.y < grid_dimensions.y / 2 {
                quadrant_counts[0] += 1;
            } else if robot.pos.y > (grid_dimensions.y / 2) {
                quadrant_counts[2] += 1;
            }
        } else if robot.pos.x > grid_dimensions.x / 2 {
            if robot.pos.y < grid_dimensions.y / 2 {
                quadrant_counts[1] += 1;
            } else if robot.pos.y > (grid_dimensions.y / 2) {
                quadrant_counts[3] += 1;
            }
        }
    }
    quadrant_counts
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Robot {
    pos: Vec2<i64>,
    vel: Vec2<i64>,
}

fn parse_robot(nums: &FourNums) -> Robot {
    Robot {
        pos: Vec2::new(nums.0, nums.1),
        vel: Vec2::new(nums.2, nums.3),
    }
}

fn wrap_pos(mut pos: Vec2<i64>, dim: Vec2<i64>) -> Vec2<i64> {
    pos.x = ((pos.x % dim.x) + dim.x) % dim.x;
    pos.y = ((pos.y % dim.y) + dim.y) % dim.y;
    pos
}

#[allow(dead_code)]
fn print_grid(robots: &[Robot], dim: Vec2<i64>) {
    let mut grid = Grid::<i32>::new(dim.x as isize, dim.y as isize);
    for robot in robots.iter() {
        grid[(robot.pos.x as isize, robot.pos.y as isize)] += 1;
    }

    for y in 0..dim.y {
        for x in 0..dim.x {
            let tile = grid[(x as isize, y as isize)];
            if tile == 0 {
                print!(".");
            } else if tile > 9 {
                print!("*");
            } else {
                print!("{}", tile);
            }
        }
        println!();
    }
    println!();
}
