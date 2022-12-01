pub mod day1;
pub mod utils;

fn main() {
    solve_day_1();
}

fn solve_day_1() {
    let res = utils::file::read_input("day1".to_string());

    println!("Max calaries carried by any elf: {}", day1::solve_part_1(&res));
    println!("Max calaries carried by top 3 elves: {}", day1::solve_part_2(&res));
}
