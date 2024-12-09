use std::fs;

use regex::Regex;

const PART1_REGEX: &str = r"mul\([0-9]{1,3},[0-9]{1,3}\)";
const PART2_REGEX: &str = r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)";

fn main() {
    let part1_lines = get_input(false, PART1_REGEX);
    let part2_lines = get_input(false, PART2_REGEX);

    println!("Part 1 Result: {}", part1(&part1_lines));
    println!("Part 2 Result: {}", part2(&part2_lines));
}

fn part1(matches: &Vec<String>) -> usize {
    matches
        .iter()
        .filter_map(|mul| mul[4..mul.len() - 1].split_once(','))
        .map(|(left, right)| left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap())
        .sum()
}

fn part2(matches: &Vec<String>) -> usize {
    let mut parse = true;

    matches
        .iter()
        .filter_map(|regex_match| {
            if *regex_match == "don't()" {
                parse = false;
                return None;
            }

            if *regex_match == "do()" {
                parse = true;
                return None;
            }

            if !parse {
                return None;
            }

            regex_match[4..regex_match.len() - 1].split_once(',')
        })
        .map(|(left, right)| left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap())
        .sum()
}

fn get_input(test: bool, regex: &str) -> Vec<String> {
    let path = if test { "test" } else { "input" };

    let contents = fs::read_to_string(format!("./.{}", path))
        .expect("Should have been able to read the file");

    let regex = Regex::new(regex).unwrap();

    regex
        .find_iter(&contents)
        .map(|regex_match| regex_match.as_str().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let lines = get_input(true, PART1_REGEX);

        let result = part1(&lines);
        assert_eq!(result, 161);
    }

    #[test]
    fn part2_works() {
        let lines = get_input(true, PART2_REGEX);

        let result = part2(&lines);
        assert_eq!(result, 48);
    }
}