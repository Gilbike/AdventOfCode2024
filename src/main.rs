use std::{collections::HashMap, fs};

fn main() {
    let test_input = get_test_input();
    let input = get_input();

    println!(
        "Test Part 1 result: {}",
        part1(&test_input.0, &test_input.1)
    );
    println!("Part 1 result: {}", part1(&input.0, &input.1));
    // println!("Test Part 2 result: {}", part2());
    // println!("Part 2 result: {}", part2());
}

fn part1(matrix: &Vec<Vec<char>>, start_postion: &(usize, usize)) -> usize {
    let mut location = (start_postion.0 as i32, start_postion.1 as i32);
    let mut direction = (0, -1);

    let mut visited: HashMap<(i32, i32), ()> = HashMap::new();

    while !is_out_of_bound(location, matrix[0].len() as i32, matrix.len() as i32) {
        visited.insert(location, ());

        let next_pos = (location.0 + direction.0, location.1 + direction.1);
        if get_char_at_pos(matrix, next_pos) == '#' {
            direction = (-direction.1, direction.0);
        }

        location.0 += direction.0;
        location.1 += direction.1;
    }

    visited.keys().count()
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

        assert_eq!(part1(&test_input.0, &test_input.1), 41)
    }
}
