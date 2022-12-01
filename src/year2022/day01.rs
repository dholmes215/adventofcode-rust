use adventofcode_rust::aoc::*;

pub fn day01(input: &str) -> SolutionResult {
    let input_lines = input.split("\n");

    let mut elf_snacks: Vec<Vec<i32>> = vec!(vec!());
    for line in input_lines {
        if line.is_empty() {
            elf_snacks.push(vec!());
        } else {
            let snack = line.parse::<i32>().unwrap();
            let mut elf = elf_snacks.pop().unwrap();
            elf.push(snack);
            elf_snacks.push(elf);
        }
    }

    let mut elf_calories = elf_snacks.iter().map(|list| list.iter().sum()).collect::<Vec<i32>>();
    elf_calories.select_nth_unstable_by(2, |a,b| b.cmp(a));
    elf_calories.resize(3, 0);

    let result = SolutionResult {
        a: elf_calories.iter().max().unwrap().to_string(),
        b: elf_calories.iter().sum::<i32>().to_string(),
    };
    result
}