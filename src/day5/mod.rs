use itertools::Itertools;
use regex::{Regex, CaptureMatches};


pub fn solve_part_1(input: &String) -> String {
    let (mut stacks, instructions) = parse_initial_stacks_and_instructions(input);
    let re = Regex::new(r"(?m)([\d+]+)").unwrap();

    instructions.lines()
        .for_each(|line| {
            let moves = get_movement_instruction(re.captures_iter(line));
            let mut range = moves.0;
            while range > 0 {
                let top = stacks[moves.1].pop().unwrap();
                stacks[moves.2].push(top);
                range -= 1;
            }
        });

    stacks.into_iter()
        .fold("".to_string(), |mut tops, mut s| {
            tops.push_str(&s.pop().unwrap().to_string());
            tops
        })
}

pub fn solve_part_2(input: &str) -> String {
    let (mut stacks, instructions) = parse_initial_stacks_and_instructions(input);
    let re = Regex::new(r"(?m)([\d+]+)").unwrap();

    instructions.lines()
        .for_each(|line| {
            let moves = get_movement_instruction(re.captures_iter(line));
            let removal_at = stacks[moves.1].len() - moves.0;
            let removed = stacks[moves.1].split_off(removal_at);
            stacks[moves.2].extend(removed);
        });

    stacks.into_iter()
        .fold("".to_string(), |mut tops, mut s| {
            tops.push_str(&s.pop().unwrap().to_string());
            tops
        })
}

fn parse_initial_stacks_and_instructions(input: &str) -> (Vec::<Vec::<char>>, &str) {
    let mut cargo = input.split("\n\n");
    let stacks = build_initial_stacks(cargo.next().unwrap());

    let instructions = cargo.next().unwrap();

    ( stacks, instructions )
}

fn build_initial_stacks(stack_representation: &str) -> Vec::<Vec::<char>> {
    stack_representation.lines()
        .rev()
        .skip(1)
        .fold(Vec::<Vec::<char>>::with_capacity(10), |mut accum, line| {
            line.chars()
                .chunks(4)
                .into_iter()
                .enumerate()
                .for_each(|(i, group)| {
                    if accum.get(i).is_none() {
                        accum.push(Vec::new());
                    }

                    let item = group.skip(1).next().unwrap();
                    if item != ' ' {
                        accum[i].push(item);
                    }
                });
            accum
        })
}

fn get_movement_instruction(captures: CaptureMatches) -> (usize, usize, usize) {
    let (range, from, to) = captures.collect_tuple().unwrap();
    (
        range.get(1).unwrap().as_str().parse::<usize>().ok().unwrap(),
        from.get(1).unwrap().as_str().parse::<usize>().ok().unwrap() - 1,
        to.get(1).unwrap().as_str().parse::<usize>().ok().unwrap() - 1,
    )
}