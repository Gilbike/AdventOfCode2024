use std::fs;

pub fn get_test_input() -> Vec<u32> {
    parse_input(true)
}

pub fn get_input() -> Vec<u32> {
    parse_input(false)
}

fn parse_input(test: bool) -> Vec<u32> {
    let file_name = if test { "./.test" } else { "./.input" };

    fs::read_to_string(file_name)
        .unwrap()
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect()
}
