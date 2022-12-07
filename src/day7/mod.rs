use std::{collections::HashMap, iter::{self, Peekable}};

pub fn solve_part_1(input: &str) -> usize {
    get_sum_of_all_directories_sizes(&mut input.lines().peekable())
        .into_iter()
        .fold(0, |mut accum, d| {
            if d.1 < 100_000 {
                accum += d.1;
            }
            accum
        })
}

pub fn solve_part_2(input: &str) -> usize {
    let dirs = get_sum_of_all_directories_sizes(&mut input.lines().peekable());
    let root_size = dirs["/"];
    let delete_at_least_this_size = 30_000_000 - (70_000_000 - root_size);

    dirs.into_iter()
        .filter(|d| d.1 >= delete_at_least_this_size)
        .map(|d| d.1)
        .min()
        .unwrap()
}

fn get_sum_of_all_directories_sizes(p_lines: &mut Peekable<std::str::Lines<'_>>) -> HashMap::<String, usize> {
    let mut directory_stack = Vec::<usize>::new();
    let mut directory_path = Vec::<&'_ str>::new();
    let mut directory_map = HashMap::<String, usize>::new();

    while let Some(instruction_or_contents) = p_lines.next() {
        if instruction_or_contents.starts_with("$ cd") {
            let dir = instruction_or_contents.split(" ").skip(2).next().unwrap();
            if dir == ".." {
                let size = directory_stack.pop().unwrap();

                directory_map
                    .entry(directory_path.join("/"))
                    .and_modify(|m| *m += size)
                    .or_insert(size);

                *directory_stack.last_mut().unwrap() += size;

                directory_path.pop();
            } else {
                directory_path.push(dir);
                directory_stack.push(0);
            }
        } else if instruction_or_contents == "$ ls" {
            // next lines are the directory listing
            let size: usize = iter::from_fn(|| p_lines.next_if(|l| !l.starts_with("$")))
                .filter(|l| !l.starts_with("dir"))
                .map(|l| l.split(" ").next().unwrap().parse::<usize>().unwrap())
                .sum();

            *directory_stack.last_mut().unwrap() += size;
        }
    }

    let loop_range = std::ops::Range { start: 0, end: directory_stack.len() };
    for _ in loop_range {
        let size = directory_stack.pop().unwrap();

        directory_map
            .entry(directory_path.join("/"))
            .and_modify(|m| *m += size)
            .or_insert(size);

        if let Some(last) = directory_stack.last_mut() {
            *last += size;
        }

        directory_path.pop();
    }

    directory_map
}
