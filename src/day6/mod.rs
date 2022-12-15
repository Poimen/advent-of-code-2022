use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> usize {
    match find_index_for_unique_chars_window(input, 4) {
        Some(i) => i,
        None => panic!("There should be a SOP indicator")
    }
}

pub fn solve_part_2(input: &str) -> usize {
    match find_index_for_unique_chars_window(input, 14) {
        Some(i) => i,
        None => panic!("There should be a SOM indicator")
    }
}

fn find_index_for_unique_chars_window(input: &str, window: usize) -> Option<usize> {
    let char_vec: Vec<char> = input.chars().collect();

    for (i, m) in char_vec.windows(window).enumerate() {
        let sop: HashSet::<&char> = m.into_iter().collect();
        if sop.len() == window {
            return Some(i + window);
        }
    }

    None
}