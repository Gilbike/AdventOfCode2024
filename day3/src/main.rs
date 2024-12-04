use std::fs;

use regex::Regex;

fn main() {
    let lines = get_input(false);

    println!("Part 1 Result: {}", part1(&lines));
    // println!("Part 2 Result: {}", part2(&lines));
}

fn part1(matches: &Vec<String>) -> usize {
    matches
        .iter()
        .filter_map(|mul| mul[4..mul.len() - 1].split_once(','))
        .map(|(left, right)| left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap())
        .sum()
}

fn get_input(test: bool) -> Vec<String> {
    let path = if test { "test" } else { "input" };

    let contents = fs::read_to_string(format!("./.{}", path))
        .expect("Should have been able to read the file");

    let regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

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
        let lines = get_input(true);

        let result = part1(&lines);
        assert_eq!(result, 161);
    }

    #[test]
    fn part2_works() {
        // let lines = get_input(true);

        // let result = part2(&lines);
        // assert_eq!(result, 4);
    }
}