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

fn part1(map: &Vec<Vec<char>>) -> usize {
    let mut radar_frequencies: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashMap<(usize, usize), ()> = HashMap::new();

    let width = map[0].len();
    let height = map.len();

    map.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(column, character)| {
            if *character != '.' {
                radar_frequencies
                    .entry(*character)
                    .or_default()
                    .push((row, column));
            }
        });
    });

    radar_frequencies
        .iter()
        .map(|(_, locations)| {
            locations
                .iter()
                .flat_map(|(row, column)| {
                    locations
                        .iter()
                        .filter_map(|(other_row, other_column)| {
                            if other_row == row && other_column == column {
                                return None;
                            }

                            let diff_y = *row as i32 - *other_row as i32;
                            let diff_x = *column as i32 - *other_column as i32;

                            let node_y = *row as i32 + diff_y as i32;
                            let node_x = *column as i32 + diff_x as i32;

                            if node_y < 0 || node_y >= height as i32 {
                                return None;
                            }

                            if node_x < 0 || node_x >= width as i32 {
                                return None;
                            }

                            if let Some(()) = antinodes.get(&(node_y as usize, node_x as usize)) {
                                return None;
                            }

                            antinodes.insert((node_y as usize, node_x as usize), ());

                            Some((node_y, node_x))
                        })
                        .collect::<Vec<(i32, i32)>>()
                })
                .count()
        })
        .sum()
}

fn part2(map: &Vec<Vec<char>>) -> usize {
    let mut radar_count = 0;
    let mut radar_frequencies: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashMap<(usize, usize), ()> = HashMap::new();

    let width = map[0].len();
    let height = map.len();

    map.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(column, character)| {
            if *character != '.' {
                radar_count += 1;

                radar_frequencies
                    .entry(*character)
                    .or_default()
                    .push((row, column));
            }
        });
    });

    let result = radar_frequencies
        .iter()
        .map(|(_, locations)| {
            locations
                .iter()
                .map(|(row, column)| {
                    locations
                        .iter()
                        .map(|(other_row, other_column)| {
                            if other_row == row && other_column == column {
                                return 0;
                            }

                            let diff_y = *row as i32 - *other_row as i32;
                            let diff_x = *column as i32 - *other_column as i32;

                            let mut node_y = *row as i32 + diff_y as i32;
                            let mut node_x = *column as i32 + diff_x as i32;

                            let mut nodes = 0;

                            while node_y >= 0
                                && node_y < height as i32
                                && node_x >= 0
                                && node_x < width as i32
                            {
                                if antinodes.get(&(node_y as usize, node_x as usize)).is_none()
                                    && map[node_y as usize][node_x as usize] == '.'
                                {
                                    nodes += 1;
                                    antinodes.insert((node_y as usize, node_x as usize), ());
                                }

                                node_y += diff_y as i32;
                                node_x += diff_x as i32;
                            }

                            nodes
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    result + radar_count
}

#[test]
fn part1_works() {
    let test_input = get_test_input();

    assert_eq!(part1(&test_input), 14);
}
