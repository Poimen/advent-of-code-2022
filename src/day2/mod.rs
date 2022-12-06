
pub fn solve_part_1(input: &str) -> i32 {
    input.lines()
        .map(calculate_strategy_points)
        .sum()
}

pub fn solve_part_2(input: &str) -> i32 {
    input.lines()
        .map(calculate_desired_strategy)
        .sum()
}

fn calculate_strategy_points(choices: &str) -> i32 {
    match choices {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,

        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,

        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,

        &_ => panic!("There should always be valid choices")
    }
}

fn calculate_desired_strategy(choices: &str) -> i32 {
    match choices {
        // Rock
        "A X" => 0 + 3,     // Lose
        "A Y" => 3 + 1,     // Draw
        "A Z" => 6 + 2,     // Win

        // Paper
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,

        // Scissors
        "C X" => 0 + 2,
        "C Y" => 3 + 3,
        "C Z" => 6 + 1,

        &_ => panic!("There should always be valid choices")
    }
}