pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod utils;

fn main() {
    let mut res = utils::file::read_input("day1");

    println!("Max calaries carried by any elf: {}", day1::solve_part_1(&res));
    println!("Max calaries carried by top 3 elves: {}", day1::solve_part_2(&res));

    res = utils::file::read_input("day2");

    println!("Elf strategy guide score: {}", day2::solve_part_1(&res));
    println!("Elf win/lose strategy guide score: {}", day2::solve_part_2(&res));

    res = utils::file::read_input("day3");

    println!("Sum of rucksack priorities: {}", day3::solve_part_1(&res));
    println!("Sum of rucksack priorities for badge groups: {}", day3::solve_part_2(&res));

    res = utils::file::read_input("day4");

    println!("Fully contained assignment pairs: {}", day4::solve_part_1(&res));
    println!("Number of overlapping assignments: {}", day4::solve_part_2(&res));

    res = utils::file::read_input("day5");

    println!("Top crates on each stack with CrateMover 9000: {}", day5::solve_part_1(&res));
    println!("Top crates on each stack with CrateMover 9001: {}", day5::solve_part_2(&res));
}

// Output
// Max calaries carried by any elf: 67027
// Max calaries carried by top 3 elves: 197291
// Elf strategy guide score: 11063
// Elf win/lose strategy guide score: 10349
// Sum of rucksack priorities: 8123
// Sum of rucksack priorities for badge groups: 2620
// Fully contained assignment pairs: 507
// Number of overlapping assignments: 897
// Top crates on each stack with CrateMover 9000: CWMTGHBDW
// Top crates on each stack with CrateMover 9001: SSCGWJCRB