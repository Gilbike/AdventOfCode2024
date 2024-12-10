use std::{collections::HashMap, fs};

fn main() {
    let mut test_input = get_test_input();
    let mut input = get_input();

    let test_result = part1(&test_input.0, &test_input.1);
    let result = part1(&input.0, &input.1);

    println!("Test Part 1 result: {}", test_result.len());
    println!("Part 1 result: {}", result.len());
    println!(
        "Test Part 2 result: {}",
        part2(&mut test_input.0, &test_input.1, &test_result)
    );
    println!("Part 2 result: {}", part2(&mut input.0, &input.1, &result));
}

fn part1(matrix: &Vec<Vec<char>>, start_postion: &(usize, usize)) -> Vec<(i32, i32)> {
    let mut location = (start_postion.0 as i32, start_postion.1 as i32);
    let mut direction = (0, -1);

    let mut visited: HashMap<(i32, i32), ()> = HashMap::new();

    while !is_out_of_bound(location, matrix[0].len() as i32, matrix.len() as i32) {
        visited.insert(location, ());

        let next_pos = (location.0 + direction.0, location.1 + direction.1);
        let right_pos = (location.0 - direction.1, location.1 + direction.0);
        if get_char_at_pos(matrix, next_pos) == '#' {
            direction = (-direction.1, direction.0);
        }

        if get_char_at_pos(matrix, next_pos) == '#' && get_char_at_pos(matrix, right_pos) == '#' {
            direction = (-direction.1, direction.0);
        }

        location.0 += direction.0;
        location.1 += direction.1;
    }

    visited
        .keys()
        .map(|key| (key.0, key.1))
        .collect::<Vec<(i32, i32)>>()
}

fn part2(
    matrix: &mut Vec<Vec<char>>,
    start_postion: &(usize, usize),
    positions: &Vec<(i32, i32)>,
) -> usize {
    let mut last = (-1, -1);

    positions
        .iter()
        .filter_map(|position| {
            matrix[position.1 as usize][position.0 as usize] = '#';

            if last.0 != -1 && last.1 != -1 {
                matrix[last.1 as usize][last.0 as usize] = '.';
            }

            last = (position.0, position.1);

            if is_looped(matrix, start_postion, positions.len() * 5) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn is_looped(matrix: &Vec<Vec<char>>, start_postion: &(usize, usize), max_steps: usize) -> bool {
    let mut location = (start_postion.0 as i32, start_postion.1 as i32);
    let mut direction = (0, -1);

    let mut visited: HashMap<(i32, i32, i32, i32), ()> = HashMap::new();

    let mut index = 0;

    while !is_out_of_bound(location, matrix[0].len() as i32, matrix.len() as i32)
        && index <= max_steps
    {
        if let Some(()) = visited.insert((location.0, location.1, direction.0, direction.1), ()) {
            return true;
        }

        let next_pos = (location.0 + direction.0, location.1 + direction.1);
        let right_pos = (location.0 - direction.1, location.1 + direction.0);
        if get_char_at_pos(matrix, next_pos) == '#' {
            direction = (-direction.1, direction.0);
        }

        if get_char_at_pos(matrix, next_pos) == '#' && get_char_at_pos(matrix, right_pos) == '#' {
            direction = (-direction.1, direction.0);
        }

        location.0 += direction.0;
        location.1 += direction.1;

        index += 1;
    }

    false
}

fn is_out_of_bound(position: (i32, i32), max_x: i32, max_y: i32) -> bool {
    if position.0 < 0 || position.0 >= max_x || position.1 < 0 || position.1 >= max_y {
        true
    } else {
        false
    }
}

fn get_char_at_pos(matrix: &Vec<Vec<char>>, position: (i32, i32)) -> char {
    if is_out_of_bound(position, matrix[0].len() as i32, matrix.len() as i32) {
        return '.';
    }

    matrix[position.1 as usize][position.0 as usize]
}

fn get_input() -> (Vec<Vec<char>>, (usize, usize)) {
    parse_input(false)
}
fn get_test_input() -> (Vec<Vec<char>>, (usize, usize)) {
    parse_input(true)
}

fn parse_input(test: bool) -> (Vec<Vec<char>>, (usize, usize)) {
    let file_name = if test { "test" } else { "input" };

    let mut start_position: (usize, usize) = (0, 0);

    let characters = fs::read_to_string(format!("./.{file_name}"))
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(column, char)| {
                    if char == '^' {
                        start_position = (column, row);
                    }

                    char
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    (characters, start_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = get_test_input();

        assert_eq!(part1(&test_input.0, &test_input.1).len(), 41)
    }

    #[test]
    fn part2_works() {
        let mut test_input = get_test_input();
        let test_result = part1(&test_input.0, &test_input.1);

        assert_eq!(part2(&mut test_input.0, &test_input.1, &test_result), 6);
    }
}
