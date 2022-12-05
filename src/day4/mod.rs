use std::ops::RangeInclusive;
use itertools::Itertools;

pub fn solve_part_1(input: &String) -> u32 {
    input.lines()
        .map(is_assignment_pair_fully_contained)
        .sum()
}

pub fn solve_part_2(input: &String) -> u32 {
    input.lines()
        .map(is_assignment_overlapping)
        .sum()
}

fn is_assignment_pair_fully_contained(assignments: &str) -> u32 {
    let elves = build_assignment_ranges(assignments);

    return (
        (elves[0].start() >= elves[1].start() && elves[0].end() <= elves[1].end()) ||
        (elves[1].start() >= elves[0].start() && elves[1].end() <= elves[0].end())
    ) as u32;
}

fn is_assignment_overlapping(assignments: &str) -> u32 {
    let elves = build_assignment_ranges(assignments);

    return (
        elves[0].contains(&elves[1].start()) ||
        elves[0].contains(&(elves[1].end())) ||
        elves[1].contains(&elves[0].start()) ||
        elves[1].contains(&(elves[0].end()))
    ) as u32;
}

fn build_assignment_ranges(assignments: &str) -> Vec<RangeInclusive<u32>> {
    assignments.split(",")
        .map(|assignment| make_range(assignment))
        .collect_vec()
}


fn make_range(str: &str) -> RangeInclusive<u32> {
    let mut r = str.split("-");

    RangeInclusive::new(
        r.next().unwrap().parse().unwrap(),
        r.next().unwrap().parse().unwrap()
    )
}