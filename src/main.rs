//
// Copyright (c) 2022 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use std::fs;

mod year2021;
mod year2022;
mod year2024;

fn main() {
    // {
    //     let input = fs::read_to_string("data/2021/01.txt").unwrap().replace("\r", "");
    //     let result = year2021::day01::day01(&input);
    //     println!("2021 Day 01 Part 1: {}", result.a);
    //     println!("2021 Day 01 Part 2: {}", result.b);
    // }
    //
    // {
    //     let input = fs::read_to_string("data/2022/01.txt").unwrap().replace("\r", "");
    //     let result = year2022::day01::day01(&input);
    //     println!("2022 Day 01 Part 1: {}", result.a);
    //     println!("2022 Day 01 Part 2: {}", result.b);
    // }
    //
    // {
    //     let input = fs::read_to_string("data/2022/03.txt").unwrap().replace("\r", "");
    //     let result = year2022::day03::day03(&input);
    //     println!("2022 Day 03 Part 1: {}", result.a);
    //     println!("2022 Day 03 Part 2: {}", result.b);
    // }
    //
    // {
    //     let input = fs::read_to_string("data/2022/04.txt").unwrap().replace("\r", "");
    //     let result = year2022::day04::day04(&input);
    //     println!("2022 Day 04 Part 1: {}", result.a);
    //     println!("2022 Day 04 Part 2: {}", result.b);
    // }
    //
    // {
    //     let input = fs::read_to_string("data/2022/06.txt").unwrap().replace("\r", "");
    //     let result = year2022::day06::day06(&input);
    //     println!("2022 Day 06 Part 1: {}", result.a);
    //     println!("2022 Day 06 Part 2: {}", result.b);
    // }

    // {
    //     let mut input_bytes = fs::read("data/2024/01.txt").unwrap();
    //     input_bytes.retain(|&x| x != ('\r' as u8));
    //     let input = std::str::from_utf8(&*input_bytes).unwrap();
    //     let result = year2024::day01::day01(&input);
    //     println!("2024 Day 01 Part 1: {}", result.a);
    //     println!("2024 Day 01 Part 2: {}", result.b);
    // }

    // {
    //     let mut input_bytes = fs::read("data/2024/02.txt").unwrap();
    //     input_bytes.retain(|&x| x != ('\r' as u8));
    //     let input = std::str::from_utf8(&*input_bytes).unwrap();
    //     let result = year2024::day02::day02(&input);
    //     println!("2024 Day 02 Part 1: {}", result.a);
    //     println!("2024 Day 02 Part 2: {}", result.b);
    // }

    {
        let mut input_bytes = fs::read("data/2024/03.txt").unwrap();
        input_bytes.retain(|&x| x != ('\r' as u8));
        let input = std::str::from_utf8(&*input_bytes).unwrap();
        let result = year2024::day03::day03(&input);
        println!("2024 Day 03 Part 1: {}", result.a);
        println!("2024 Day 03 Part 2: {}", result.b);
    }
}
