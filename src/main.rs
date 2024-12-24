// Copyright (c) 2022 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

extern crate core;

use std::fs;

mod year2021;
mod year2022;
mod year2024;

fn main() {
    // {
    //     let input = fs::read_to_string("data/2021/01.txt").unwrap().replace('\r', "");
    //     let result = year2021::day01::day01(&input);
    //     println!("2021 Day 01 Part 1: {}", result.a);
    //     println!("2021 Day 01 Part 2: {}", result.b);
    // }
    //
    // {
    //     let input = fs::read_to_string("data/2022/01.txt").unwrap().replace('\r', "");
    //     let result = year2022::day01::day01(&input);
    //     println!("2022 Day 01 Part 1: {}", result.a);
    //     println!("2022 Day 01 Part 2: {}", result.b);
    // }
    //
    // {
    //     let input = fs::read_to_string("data/2022/03.txt").unwrap().replace('\r', "");
    //     let result = year2022::day03::day03(&input);
    //     println!("2022 Day 03 Part 1: {}", result.a);
    //     println!("2022 Day 03 Part 2: {}", result.b);
    // }
    //
    // {
    //     let input = fs::read_to_string("data/2022/04.txt").unwrap().replace('\r', "");
    //     let result = year2022::day04::day04(&input);
    //     println!("2022 Day 04 Part 1: {}", result.a);
    //     println!("2022 Day 04 Part 2: {}", result.b);
    // }
    //
    // {
    //     let input = fs::read_to_string("data/2022/06.txt").unwrap().replace('\r', "");
    //     let result = year2022::day06::day06(&input);
    //     println!("2022 Day 06 Part 1: {}", result.a);
    //     println!("2022 Day 06 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/01.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day01::day01(input);
    //     println!("2024 Day 01 Part 1: {}", result.a);
    //     println!("2024 Day 01 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/02.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day02::day02(input);
    //     println!("2024 Day 02 Part 1: {}", result.a);
    //     println!("2024 Day 02 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/03.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day03::day03(input);
    //     println!("2024 Day 03 Part 1: {}", result.a);
    //     println!("2024 Day 03 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/04.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     // let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day04::day04(&input_bytes);
    //     println!("2024 Day 04 Part 1: {}", result.a);
    //     println!("2024 Day 04 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/05.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day05::day05(input);
    //     println!("2024 Day 05 Part 1: {}", result.a);
    //     println!("2024 Day 05 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/06.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day06::day06(input);
    //     println!("2024 Day 06 Part 1: {}", result.a);
    //     println!("2024 Day 06 Part 2: {}", result.b);
    // }
    // 
    // {
    //     let mut input_bytes = fs::read("data/2024/07.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day07::day07(input);
    //     println!("2024 Day 07 Part 1: {}", result.a);
    //     println!("2024 Day 07 Part 2: {}", result.b);
    // }
    // 
    // {
    //     let mut input_bytes = fs::read("data/2024/08.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day08::day08(input);
    //     println!("2024 Day 08 Part 1: {}", result.a);
    //     println!("2024 Day 08 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/09.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day09::day09(input);
    //     println!("2024 Day 09 Part 1: {}", result.a);
    //     println!("2024 Day 09 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/10.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day10::day10(input);
    //     println!("2024 Day 10 Part 1: {}", result.a);
    //     println!("2024 Day 10 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/11.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day11::day11(input);
    //     println!("2024 Day 11 Part 1: {}", result.a);
    //     println!("2024 Day 11 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/12.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day12::day12(input);
    //     println!("2024 Day 12 Part 1: {}", result.a);
    //     println!("2024 Day 12 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/13.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day13::day13(input);
    //     println!("2024 Day 13 Part 1: {}", result.a);
    //     println!("2024 Day 13 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/14.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day14::day14(input);
    //     println!("2024 Day 14 Part 1: {}", result.a);
    //     println!("2024 Day 14 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/15.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day15::day15(input);
    //     println!("2024 Day 15 Part 1: {}", result.a);
    //     println!("2024 Day 15 Part 2: {}", result.b);
    // }
    // 
    // {
    //     let mut input_bytes = fs::read("data/2024/17.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day17::day17(input);
    //     println!("2024 Day 17 Part 1: {}", result.a);
    //     println!("2024 Day 17 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/18.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day18::day18(input);
    //     println!("2024 Day 18 Part 1: {}", result.a);
    //     println!("2024 Day 18 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/19.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day19::day19(input);
    //     println!("2024 Day 19 Part 1: {}", result.a);
    //     println!("2024 Day 19 Part 2: {}", result.b);
    // }
    //
    // {
    //     let mut input_bytes = fs::read("data/2024/20.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day20::day20(input);
    //     println!("2024 Day 20 Part 1: {}", result.a);
    //     println!("2024 Day 20 Part 2: {}", result.b);
    // }
    // 
    // {
    //     let mut input_bytes = fs::read("data/2024/21.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day21::day21(input);
    //     println!("2024 Day 21 Part 1: {}", result.a);
    //     println!("2024 Day 21 Part 2: {}", result.b);
    // }
    // 
    // {
    //     let mut input_bytes = fs::read("data/2024/22.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day22::day22(input);
    //     println!("2024 Day 22 Part 1: {}", result.a);
    //     println!("2024 Day 22 Part 2: {}", result.b);
    // }
    // 
    // {
    //     let mut input_bytes = fs::read("data/2024/23.txt").unwrap();
    //     input_bytes.retain(|&x| x != b'\r');
    //     let input = std::str::from_utf8(&input_bytes).unwrap();
    //     let result = year2024::day23::day23(input);
    //     println!("2024 Day 23 Part 1: {}", result.a);
    //     println!("2024 Day 23 Part 2: {}", result.b);
    // }

    {
        let mut input_bytes = fs::read("data/2024/24example2.txt").unwrap();
        input_bytes.retain(|&x| x != b'\r');
        let input = std::str::from_utf8(&input_bytes).unwrap();
        let result = year2024::day24::day24(input);
        println!("2024 Day 24 Part 1: {}", result.a);
        println!("2024 Day 24 Part 2: {}", result.b);
    }
}
