use std::fs;

mod year2021;

fn main() {
    let input = fs::read_to_string("data/2021/01.txt").unwrap();

    let result = year2021::day01::day01(&input);
    std::println!("Part 1: {}", result.a);
    std::println!("Part 2: {}", result.b);
}
