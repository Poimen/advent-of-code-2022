
pub fn solve_part_1(input: &str) -> i32 {

    let mut x = 1;
    let mut cycle_count = 0;

    input.lines()
        .into_iter()
        .fold(0, |mut accum, l| {
            let (cycle_length, val) = parse_instruction(l);

            for _ in 0..cycle_length {
                cycle_count += 1;
                if let 20 | 60 | 100 | 140 | 180 | 220 = cycle_count {
                     accum += cycle_count * x;
                }
            }

            x += val;
            accum
        })
}

pub fn solve_part_2(input: &str) {
    let mut sprite_x = 1;
    let mut crt_pos = 0;

    input.lines()
        .into_iter()
        .fold(Vec::<String>::new(), |mut accum, l| {
            let (cycle_length, val) = parse_instruction(l);
            for _ in 0..cycle_length {
                let pos: i32 = (crt_pos % 40) - sprite_x;
                if pos.abs() <= 1 {
                    accum.push("#".to_string());
                } else {
                    accum.push(" ".to_string());
                }
                crt_pos += 1;
            }

            sprite_x += val;
            accum
        })
        .chunks(40)
        .into_iter()
        .for_each(|c| {
            println!("{:?}", c.join(""))
        });
}

fn parse_instruction(instruction: &str) -> (usize, i32) {
    if instruction.starts_with("addx") {
        return (
            2,
            instruction.split(" ").skip(1).next().unwrap().parse::<i32>().unwrap()
        );
    }

    return (1, 0);
}