use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

pub fn solve_part_1(input: &str) -> usize {
    let mut current_head = Point { x: 0, y: 0 };
    let mut current_tail = Point { x: 0, y: 0 };
    let mut tail_movements = HashSet::new();

    get_head_movement(input)
        .into_iter()
        .for_each(|h| {
            current_head = move_to(&current_head, h);
            current_tail = move_tail_to(&current_tail, &current_head);
            tail_movements.insert(current_tail);
        });

    tail_movements.len()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut current_head = Point { x: 0, y: 0 };
    let mut rope = vec![Point { x: 0, y: 0 }; 9];
    let mut tail_movements = HashSet::<Point>::new();

    get_head_movement(input)
        .into_iter()
        .for_each(|h| {
            current_head = move_to(&current_head, h);

            rope[0] = move_tail_to(&rope[0], &current_head);

            for i in 1..9 {
                rope[i] = move_tail_to(&rope[i], &rope[i-1]);
            }

            tail_movements.insert(*rope.last().unwrap());
        });

    tail_movements.len()
}

fn move_to(s: &Point, p: Point) -> Point {
    Point {
        x: s.x + p.x,
        y: s.y + p.y
    }
}

fn move_tail_to(s: &Point, head: &Point) -> Point {
    let diff = Point { x: head.x - s.x, y: head.y - s.y };

    if diff.x.abs() > 1 || diff.y.abs() > 1 {
        Point {
            x: s.x + calculate_axis_movement(diff.x),
            y: s.y + calculate_axis_movement(diff.y)
        }
    } else {
        Point { x: s.x, y: s.y }
    }
}

fn calculate_axis_movement(coord: i32) -> i32 {
    if coord == 0 {
        0
    } else if coord < 0 {
        -1
    } else {
        1
    }
}

fn get_head_movement(input: &str) -> Vec<Point> {
    input.lines()
        .fold(Vec::<Point>::new(), |mut accum, m| {
            let mut l = m.split(" ");
            let dir = l.next().unwrap();
            let count = l.next().unwrap().parse::<usize>().unwrap();

            for _ in 0..count {
                accum.push(
                    match dir {
                        "R" => Point { x:  1, y:  0 },
                        "L" => Point { x: -1, y:  0 },
                        "U" => Point { x:  0, y:  1 },
                        "D" => Point { x:  0, y: -1 },
                        _ => panic!("Unknown direction")
                    }
                )
            }

            accum
        })
}