use itertools::Itertools;

pub fn solve_part_1(input: &String) -> u32 {
    input.lines()
        .map(calculate_rucksack_priority)
        .sum()
}

pub fn solve_part_2(input: &String) -> u32 {
    input.lines()
        .tuples::<(_, _, _)>()
        .map(calculate_rucksack_badge_and_priority)
        .sum()
}

fn calculate_rucksack_priority(contents: &str) -> u32 {
    let search_len = contents.len() / 2;
    let search_string = contents.get(..search_len).unwrap();
    let second_half = contents.get(search_len..).unwrap();

    for c in search_string.chars() {
        if second_half.find(c).is_some() {
            return calculate_priority(c);
        }
    }

    panic!("Should always find a unique character");
}

fn calculate_priority(c: char) -> u32 {
    let lower_a_digit = 'a' as u32;
    let upper_a_digit = 'A' as u32;
    let unique_digit = c as u32;

    if c.is_lowercase() {
        return unique_digit - lower_a_digit + 1;
    }
    return unique_digit - upper_a_digit + 27;
}

fn calculate_rucksack_badge_and_priority(group: (&str, &str, &str)) -> u32 {
    for c in group.0.chars() {
        if group.1.find(c).is_some() && group.2.find(c).is_some() {
            return calculate_priority(c);
        }
    }

    return 0;
}