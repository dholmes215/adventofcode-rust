use adventofcode_rust::aoc::*;

pub fn day01(input: &str) -> SolutionResult {
    let input_lines = input.split("\n");

    let mut elf_calories: Vec<i32> = vec!(0);
    for line in input_lines {
        if line.is_empty() {
            elf_calories.push(0);
        } else {
            let snack = line.parse::<i32>().unwrap();
            *elf_calories.last_mut().unwrap() += snack;
        }
    }

    elf_calories.select_nth_unstable_by(2, |a,b| b.cmp(a));
    elf_calories.resize(3, 0);

    let result = SolutionResult {
        a: elf_calories.iter().max().unwrap().to_string(),
        b: elf_calories.iter().sum::<i32>().to_string(),
    };
    result
}