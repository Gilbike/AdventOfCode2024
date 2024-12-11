use std::fs;

fn parse_input(test: bool) -> Vec<(i64, Vec<i64>)> {
    let file_name = if test { "./.test" } else { "./.input" };
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .filter_map(|line| line.split_once(':'))
        .map(|(left, right)| {
            let raw = right.trim();

            let numbers = raw
                .split(' ')
                .map(|raw_number| raw_number.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            (left.parse::<i64>().unwrap(), numbers)
        })
        .collect()
}

pub fn get_input() -> Vec<(i64, Vec<i64>)> {
    parse_input(false)
}

pub fn get_test_input() -> Vec<(i64, Vec<i64>)> {
    parse_input(true)
}
