use std::fs::{self};

const NEIGHBOURS: [[i32; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];

fn main() {
    let test_input = get_test_input();
    let input = get_input();

    println!("Test result: {}", part1(&test_input));
    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));
}

fn part1(matrix: &Vec<Vec<char>>) -> usize {
    (0..matrix.len())
        .map(|row| {
            (0..matrix[0].len())
                .map(|column| {
                    if matrix[row][column] == 'X' {
                        NEIGHBOURS
                            .iter()
                            .filter_map(|[x, y]| {
                                is_valid_word(&matrix, [column as i32, row as i32], [*x, *y])
                                    .then_some(())
                            })
                            .count()
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn part2(matrix: &Vec<Vec<char>>) -> usize {
    (1..matrix.len() - 1)
        .map(|row| {
            (1..matrix[0].len() - 1)
                .filter_map(|column| {
                    if matrix[row][column] == 'A' {
                        if ((matrix[row - 1][column - 1] == 'M'
                            && matrix[row + 1][column + 1] == 'S')
                            || (matrix[row - 1][column - 1] == 'S'
                                && matrix[row + 1][column + 1] == 'M'))
                            && ((matrix[row - 1][column + 1] == 'M'
                                && matrix[row + 1][column - 1] == 'S')
                                || (matrix[row - 1][column + 1] == 'S'
                                    && matrix[row + 1][column - 1] == 'M'))
                        {
                            return Some(());
                        }

                        None
                    } else {
                        None
                    }
                })
                .count()
        })
        .sum()
}

fn is_valid_word(matrix: &Vec<Vec<char>>, origin: [i32; 2], direction: [i32; 2]) -> bool {
    let character = matrix[origin[1] as usize][origin[0] as usize];

    if matrix[origin[1] as usize][origin[0] as usize] == 'S' {
        return true;
    }

    if let Some(next) = get_character_in_direction(matrix, origin, direction) {
        if is_character_following(character, next).is_some() {
            return is_valid_word(
                matrix,
                [origin[0] + direction[0], origin[1] + direction[1]],
                direction,
            );
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn get_character_in_direction(
    matrix: &Vec<Vec<char>>,
    origin: [i32; 2],
    direction: [i32; 2],
) -> Option<char> {
    let x_cord = origin[0] + direction[0];
    let y_cord = origin[1] + direction[1];

    if x_cord >= matrix[0].len() as i32 || x_cord < 0 {
        return None;
    }

    if y_cord >= matrix.len() as i32 || y_cord < 0 {
        return None;
    }

    Some(matrix[y_cord as usize][x_cord as usize])
}

fn is_character_following(character: char, next: char) -> Option<()> {
    match next {
        'M' if character == 'X' => Some(()),
        'A' if character == 'M' => Some(()),
        'S' if character == 'A' => Some(()),
        _ => None,
    }
}

fn get_input() -> Vec<Vec<char>> {
    read_input(false)
}

fn get_test_input() -> Vec<Vec<char>> {
    read_input(true)
}

fn read_input(test: bool) -> Vec<Vec<char>> {
    let file_name = if test { "test" } else { "input" };

    fs::read_to_string(format!("./.{file_name}"))
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = get_test_input();

        assert_eq!(part1(&test_input), 18);
    }

    #[test]
    fn part2_works() {
        let test_input = get_test_input();

        assert_eq!(part2(&test_input), 9);
    }
}
