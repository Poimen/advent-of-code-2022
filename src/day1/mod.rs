use itertools::Itertools;

// Solve the max calaries any elf carries
pub fn solve_part_1(input: &str) -> i32 {
    input.split("\n\n")
        .map(get_elf_total_carrying_calaries)
        .max()
        .unwrap()
}

// Solve the max cakaries carried by top 3
pub fn solve_part_2(input: &str) -> i32 {
    input.split("\n\n")
        .map(get_elf_total_carrying_calaries)
        .sorted_by_key(|&w| std::cmp::Reverse(w))
        .take(3)
        .sum()
}

fn get_elf_total_carrying_calaries(itr: &str) -> i32 {
    itr.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>()
}