mod input;

use crate::input::{get_input, get_test_input};

fn main() {
    let test_input = get_test_input();
    let input = get_input();

    println!("Part 1 Test Result: {}", part1(&test_input));
    println!("Part 1 Result: {}", part1(&input));
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

fn solve_equasion(equasion: &str) -> i64 {
    let mut result = 0;
    let mut operation = '+';

    equasion.split_whitespace().for_each(|token| {
        if token.chars().last().unwrap() == '*' || token.chars().last().unwrap() == '+' {
            operation = token.chars().last().unwrap();
        } else {
            let number = token.parse::<i64>().unwrap();

            if operation == '+' {
                result += number;
            } else {
                result *= number;
            }
        }
    });

    result
}

#[test]
fn part1_works() {
    let test_input = get_test_input();

    assert_eq!(part1(&test_input), 3749);
}
