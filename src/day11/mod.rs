use itertools::Itertools;

struct Monkey {
    items: Vec::<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
    test_divisor: usize
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .finish()
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let mut monkeys = initialise_monkey_holdings(input);
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..20 {

        for idx in 0..monkeys.len() {
            let items = monkeys[idx].items.drain(..).collect_vec();

            items.into_iter().for_each(|worry_level| {
                inspections[idx] += 1;

                let new_worry_level = (monkeys[idx].op)(worry_level) / 3;
                let to_next_monkey = (monkeys[idx].test)(new_worry_level);
                monkeys[to_next_monkey].items.push(new_worry_level);
            });
        }
    }

    inspections.into_iter()
        .sorted_by_key(|&w| std::cmp::Reverse(w))
        .take(2)
        .product()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut monkeys = initialise_monkey_holdings(input);
    let mut inspections = vec![0; monkeys.len()];
    let common_multiple: usize = monkeys.iter().map(|m| m.test_divisor).product();

    for _ in 0..10_000 {

        for idx in 0..monkeys.len() {
            let items = monkeys[idx].items.drain(..).collect_vec();

            items.into_iter().for_each(|worry_level| {
                inspections[idx] += 1;

                let new_worry_level = (monkeys[idx].op)(worry_level) % common_multiple;
                let to_next_monkey = (monkeys[idx].test)(new_worry_level);
                monkeys[to_next_monkey].items.push(new_worry_level);
            });
        }
    }

    inspections.into_iter()
        .sorted_by_key(|&w| std::cmp::Reverse(w))
        .take(2)
        .product()
}

fn initialise_monkey_holdings(input: &str ) -> Vec::<Monkey> {
    input.split("\n\n")
        .fold(Vec::<Monkey>::new(), |mut accum, m| {
            let mut l = m.lines().skip(1);
            let items = l.next().unwrap().split(":").skip(1).next().unwrap().split(",").map(|s| {
                s.trim().parse::<usize>().unwrap()
            }).collect_vec();

            let mut operation_line = l.next().unwrap().split("=").skip(1).next().unwrap().trim().split(" ").skip(1);
            let op = make_operation(operation_line.next().unwrap(), operation_line.next().unwrap());

            let divider = l.next().unwrap().trim().split("Test: divisible by ").skip(1).next().unwrap().parse::<usize>().unwrap();
            let true_monkey = l.next().unwrap().trim().split("If true: throw to monkey ").skip(1).next().unwrap().parse::<usize>().unwrap();
            let false_monkey = l.next().unwrap().trim().split("If false: throw to monkey ").skip(1).next().unwrap().parse::<usize>().unwrap();

            accum.push(Monkey {
                items,
                op,
                test: make_tester(divider, true_monkey, false_monkey),
                test_divisor: divider
            });
            accum
        })
}

fn make_operation(operator: &str, operand: &str) -> Box<dyn Fn(usize) -> usize> {
    match operand {
        "old" => make_squarer(),
        _ => {
            let amount = operand.parse::<usize>().unwrap();
            match operator {
                "*" => make_multiplier(amount),
                "+" => make_adder(amount),
                "old" => make_squarer(),
                op => panic!("Unknown operand: {}", op)
            }
        }
    }
}

fn make_tester(divisor: usize, true_monkey: usize, false_monkey: usize) -> Box<dyn Fn(usize) -> usize> {
    Box::new(move |worry_level| {
        match worry_level % divisor {
            0 => true_monkey,
            _ => false_monkey
        }
    })
}

fn make_adder(amount: usize) -> Box<dyn Fn(usize) -> usize> {
    Box::new(move |old| old + amount)
}

fn make_multiplier(amount: usize) -> Box<dyn Fn(usize) -> usize> {
    Box::new(move |old| old * amount)
}

fn make_squarer() -> Box<dyn Fn(usize) -> usize> {
    Box::new(move |old| old * old)
}
