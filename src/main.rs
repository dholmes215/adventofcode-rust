//
// Copyright (c) 2022 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use std::fs;

mod year2021;
mod year2022;

fn main() {
    {
        let input = fs::read_to_string("data/2021/01.txt").unwrap();
        let result = year2021::day01::day01(&input);
        std::println!("2021 Day 01 Part 1: {}", result.a);
        std::println!("2021 Day 01 Part 2: {}", result.b);
    }

    {
        let input = fs::read_to_string("data/2022/01.txt").unwrap();
        let result = year2022::day01::day01(&input);
        std::println!("2022 Day 01 Part 1: {}", result.a);
        std::println!("2022 Day 01 Part 2: {}", result.b);
    }
}
