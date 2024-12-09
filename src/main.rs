use std::{collections::HashMap, fs};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() {
    let test_input = get_test_input();
    let input = get_input();

    let test_result = part1(&test_input.0, &test_input.1);
    let result = part1(&input.0, &input.1);

    println!("Part 1 Test result: {}", test_result.0);
    println!("Part 1 result: {}", result.0);
    println!(
        "Part 2 Test result: {}",
        part2(&test_input.0, &test_result.1)
    );
    println!("Part 2 result: {}", part2(&input.0, &result.1));
}

fn part1(
    rules_base: &HashMap<usize, Vec<usize>>,
    updates: &Vec<Vec<usize>>,
) -> (usize, Vec<Vec<usize>>) {
    let mut invalid_lines: Vec<Vec<usize>> = Vec::new();

    let result = updates
        .iter()
        .filter_map(|update| {
            let mut rules = rules_base.clone();

            rules.retain(|key, _| update.contains(key));

            let valid_line = update.iter().all(|number| {
                let key_count = rules
                    .iter()
                    .filter_map(|(key, val)| {
                        if val.contains(number) {
                            Some(*key)
                        } else {
                            None
                        }
                    })
                    .count();

                if key_count > 0 {
                    return false;
                }

                rules.remove(number);

                true
            });

            if !valid_line {
                invalid_lines.push(update.clone());
                return None;
            }

            Some(update)
        })
        .map(|line| line[line.len() / 2])
        .sum();

    (result, invalid_lines)
}

fn part2(rules_base: &HashMap<usize, Vec<usize>>, updates: &Vec<Vec<usize>>) -> usize {
    updates
        .iter()
        .filter_map(|update| {
            let mut rules = rules_base
                .iter()
                .filter_map(|(key, value)| {
                    if !update.contains(key) {
                        return None;
                    }

                    let new_values = value
                        .clone()
                        .iter()
                        .filter_map(|val| update.contains(val).then_some(*val))
                        .collect::<Vec<usize>>();

                    Some((*key, new_values))
                })
                .collect::<Vec<(usize, Vec<usize>)>>();

            rules.sort_by(|(_, val1), (_, val2)| val2.len().cmp(&val1.len()));

            Some(rules[rules.len() / 2].0)
        })
        .sum()
}

fn get_input() -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    read_input(false)
}

fn get_test_input() -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    read_input(true)
}

fn read_input(test: bool) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let file_name = if test { "test" } else { "input" };
    let delimiter = format!("{0}{0}", LINE_ENDING);

    let file_content = fs::read_to_string(format!("./.{file_name}")).unwrap();
    let (rules, updates) = file_content.split_once(&delimiter).unwrap();

    let mut rules_base: HashMap<usize, Vec<usize>> = HashMap::new();

    rules
        .lines()
        .filter_map(|line| line.split_once('|'))
        .for_each(|(before, after)| {
            rules_base
                .entry(before.parse::<usize>().unwrap())
                .or_default()
                .push(after.parse::<usize>().unwrap());
        });

    let updates_base = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|char| char.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    (rules_base, updates_base)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = get_test_input();

        assert_eq!(part1(&test_input.0, &test_input.1).0, 143);
    }

    #[test]
    fn part2_works() {
        let test_input = get_test_input();
        let result = part1(&test_input.0, &test_input.1);

        assert_eq!(part2(&test_input.0, &result.1), 123);
    }
}
