pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
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

    res = utils::file::read_input("day6");

    println!("Start of packet index: {}", day6::solve_part_1(&res));
    println!("Start of message index: {}", day6::solve_part_2(&res));

    res = utils::file::read_input("day7");

    println!("Sum of the total directories sized less than 100_000: {}", day7::solve_part_1(&res));
    println!("Size of the smallest directory to delete: {}", day7::solve_part_2(&res));

    res = utils::file::read_input("day8");

    println!("Number of trees visible from outside: {}", day8::solve_part_1(&res));
    println!("Best scenic score: {}", day8::solve_part_2(&res));

    res = utils::file::read_input("day9");

    println!("Number of positions the tail reached: {}", day9::solve_part_1(&res));
    println!("Number of positions the tail reached (after snap): {}", day9::solve_part_2(&res));

    res = utils::file::read_input("day10");

    println!("Sum of 6 signal strengths: {}", day10::solve_part_1(&res));
    println!("Ascii art!:");
    day10::solve_part_2(&res);

    res = utils::file::read_input("day11");

    println!("Level of monkey business after 20 rounds: {}", day11::solve_part_1(&res));
    println!("Level of monkey business after 10_000 rounds: {}", day11::solve_part_2(&res));

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
// Start of packet index: 1343
// Start of message index: 2193
// Sum of the total directories sized less than 100_000: 1306611
// Size of the smallest directory to delete: 13210366
// Number of trees visible from outside: 1792
// Best scenic score: 334880
// Number of positions the tail reached: 5960
// Number of positions the tail reached (after snap): 2327
// Sum of 6 signal strengths: 11720
// Ascii art!:
// "#### ###   ##  ###  #### ###   ##    ## "
// "#    #  # #  # #  # #    #  # #  #    # "
// "###  #  # #    #  # ###  #  # #       # "
// "#    ###  #    ###  #    ###  #       # "
// "#    # #  #  # # #  #    #    #  # #  # "
// "#### #  #  ##  #  # #### #     ##   ##  "
// Level of monkey business after 20 rounds: 119715
// Level of monkey business after 10_000 rounds: 18085004878