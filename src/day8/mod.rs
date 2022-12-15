use grid::Grid;

#[derive(Debug)]
struct Tree {
    height: usize,
    is_visible: bool
}

pub fn solve_part_1(input: &str) -> usize {
    let (size, mut forest) = build_forest_grid(input);

    for i in 1..size {
        let mut prev_height = 0;
        forest.iter_row_mut(i).take(size-1).for_each(|t| {
            if t.height > prev_height {
                t.is_visible = true;
                prev_height = t.height;
            }
        });

        prev_height = 0;
        forest.iter_row_mut(i).rev().take(size-1).for_each(|t| {
            if t.height > prev_height {
                t.is_visible = true;
                prev_height = t.height;
            }
        });

        prev_height = 0;
        forest.iter_col_mut(i).take(size-1).for_each(|t| {
            if t.height > prev_height {
                t.is_visible = true;
                prev_height = t.height;
            }
        });

        prev_height = 0;
        forest.iter_col_mut(i).rev().take(size-1).for_each(|t| {
            if t.height > prev_height {
                t.is_visible = true;
                prev_height = t.height;
            }
        });
    }

    forest.iter().filter(|t| t.is_visible).count()
}

pub fn solve_part_2(input: &str) -> usize {
    let (size, forest) = build_forest_grid(input);
    let mut max_scenic_tree = 0;


    for x in 1..size-1 {
        for y in 1..size-1 {
            let tree = &forest[x][y];
            let mut scenic_score = 1;

            let mut tree_count = 0;
            for up in (0..x).rev() {
                let next_tree = &forest[up][y];

                tree_count += 1;
                if next_tree.height >= tree.height {
                    break;
                }
            }
            scenic_score *= tree_count;

            tree_count = 0;
            for down in x+1..size {
                let next_tree = &forest[down][y];

                tree_count += 1;
                if next_tree.height >= tree.height {
                    break;
                }
            }
            scenic_score *= tree_count;

            tree_count = 0;
            for left in (0..y).rev() {
                let next_tree = &forest[x][left];

                tree_count += 1;
                if next_tree.height >= tree.height {
                    break;
                }
            }
            scenic_score *= tree_count;

            tree_count = 0;
            for right in y+1..size {
                let next_tree = &forest[x][right];

                tree_count += 1;
                if next_tree.height >= tree.height {
                    break;
                }
            }
            scenic_score *= tree_count;

            if scenic_score > max_scenic_tree {
                max_scenic_tree = scenic_score;
            }
        }
    }

    max_scenic_tree
}

fn build_forest_grid(input: &str) -> (usize, Grid<Tree>) {
    let size = input.lines().peekable().peek().unwrap().len();
    let last_row = size - 1;
    let mut forest = Vec::<Tree>::new();

    input.lines()
        .enumerate()
        .for_each(|(i, l)| {
            l.chars()
                .enumerate()
                .for_each(|(j, h)| forest.push(Tree {
                    is_visible: i == 0 || i == last_row || j == 0 || j == last_row,
                    height: h.to_digit(10).unwrap() as usize
                }));
        });

    (size, Grid::from_vec(forest, size))
}
