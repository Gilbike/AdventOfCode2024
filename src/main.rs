mod input;

use std::collections::HashMap;

use crate::input::{get_input, get_test_input};

fn main() {
    let test_input = get_test_input();
    let input = get_input();

    println!("Part 1 Test Result: {}", part1(&test_input));
    println!("Part 1 Result: {}", part1(&input));
    println!("Part 2 Test Result: {}", part2(&test_input));
    println!("Part 2 Result: {}", part2(&input));
}

fn part1(lines: &Vec<(i64, Vec<i64>)>) -> i64 {
    lines
        .iter()
        .filter_map(|(sum, numbers)| {
            let valid = (0..2i32.pow(numbers.len() as u32)).any(|index| {
                let mut equasion = String::new();

                (0..(numbers.len() - 1)).for_each(|n| {
                    let part = format!(
                        "{} {} ",
                        numbers[n].to_string(),
                        if (index >> n) & 1 == 1 { "+" } else { "*" }
                    );

                    equasion.push_str(&part);
                });

                equasion.push_str(&numbers[numbers.len() - 1].to_string());

                solve_equasion(&equasion) == *sum
            });

            if valid {
                Some(*sum)
            } else {
                None
            }
        })
        .sum()
}

fn part2(lines: &Vec<(i64, Vec<i64>)>) -> i64 {
    let mut combinations: HashMap<i64, Vec<String>> = HashMap::new();

    lines
        .iter()
        .filter_map(|(sum, numbers)| {
            let mut combos: Vec<String>;

            if let Some(c) = combinations.get(&((numbers.len() - 1) as i64)) {
                combos = c.clone();
            } else {
                combos = Vec::new();

                generate_combinations(&['*', '|', '+'], numbers.len() - 1, Vec::new(), &mut combos);

                combinations.insert((numbers.len() - 1) as i64, combos.clone());
            }

            let valid = combos.iter().any(|combo| {
                let mut equasion = String::new();

                combo.chars().enumerate().for_each(|(index, character)| {
                    let part = format!("{} {} ", numbers[index].to_string(), character);

                    equasion.push_str(&part);
                });

                equasion.push_str(&numbers[numbers.len() - 1].to_string());

                solve_equasion(&equasion) == *sum
            });

            if valid {
                Some(*sum)
            } else {
                None
            }
        })
        .sum()
}

fn solve_equasion(equasion: &str) -> i64 {
    let mut result = 0;
    let mut operation = '+';

    equasion.split_whitespace().for_each(|token| {
        if token.chars().last().unwrap() == '*'
            || token.chars().last().unwrap() == '+'
            || token.chars().last().unwrap() == '|'
        {
            operation = token.chars().last().unwrap();
        } else {
            let number = token.parse::<i64>().unwrap();

            if operation == '+' {
                result += number;
            } else if operation == '|' {
                let mut as_string = result.to_string();
                as_string.push_str(token);
                result = as_string.parse::<i64>().unwrap();
            } else {
                result *= number;
            }
        }
    });

    result
}

fn generate_combinations(
    chars: &[char],
    n: usize,
    current_combination: Vec<char>,
    combinations: &mut Vec<String>,
) {
    if current_combination.len() == n {
        combinations.push(current_combination.iter().collect::<String>());
        return;
    }

    for &ch in chars {
        let mut new_combination = current_combination.clone();
        new_combination.push(ch);
        generate_combinations(chars, n, new_combination, combinations);
    }
}

#[test]
fn part1_works() {
    let test_input = get_test_input();

    assert_eq!(part1(&test_input), 3749);
}

#[test]
fn part2_works() {
    let test_input = get_test_input();

    assert_eq!(part2(&test_input), 11387);
}
