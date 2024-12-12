use std::fs;

pub fn get_test_input() -> Vec<Vec<char>> {
    parse_input(true)
}

pub fn get_input() -> Vec<Vec<char>> {
    parse_input(false)
}

fn parse_input(test: bool) -> Vec<Vec<char>> {
    let file_path = if test { "./.test" } else { "./.input" };

    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
