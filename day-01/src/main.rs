use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut elf_calories = content
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
                .iter()
                .sum::<u64>()
        })
        .collect::<Vec<u64>>();

    elf_calories.sort();
    elf_calories.reverse();

    println!("solution 1: {}", elf_calories[0]);
    println!("solution 2: {}", elf_calories[..3].iter().sum::<u64>());
}
