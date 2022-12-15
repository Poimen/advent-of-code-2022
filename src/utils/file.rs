use std::env;
use std::fs;

pub fn read_input(day: &str) -> String {
    let file_path = env::current_dir()
        .unwrap()
        .join("src")
        .join(day)
        .join("input");

    match fs::read_to_string(&file_path) {
        Ok(input) => input,
        Err(error) => panic!("Failed to read file ({}): {:?}", file_path.display(), error)
    }
}