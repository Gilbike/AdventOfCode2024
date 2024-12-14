mod input;

use std::collections::HashMap;

use input::get_input;

use crate::input::get_test_input;

fn main() {
    let test_input = get_test_input();
    let input = get_input();

    println!("Part 1 Test Result: {}", part1(&test_input));
    println!("Part 1 Result: {}", part1(&input));
    println!("Part 2 Test Result: {}", part2(&test_input));
    println!("Part 2 Result: {}", part2(&input));
}

fn part1(digits: &Vec<u32>) -> i64 {
    let mut positions: HashMap<u32, i32> = HashMap::new();
    let mut free_positions: Vec<u32> = Vec::new();

    let mut i = 0;

    digits.iter().enumerate().for_each(|(index, digit)| {
        for _ in 0..*digit {
            if index % 2 == 0 {
                positions.insert(i, (index / 2) as i32);
            } else {
                positions.insert(i, -1);
                free_positions.push(i);
            }
            i += 1;
        }
    });

    let mut postion_vector: Vec<_> = positions
        .iter()
        .map(|(key, value)| (*key, *value))
        .collect();
    postion_vector.sort_by(|a, b| b.0.cmp(&a.0));

    postion_vector.iter().for_each(|(key, value)| {
        if *value == -1 {
            return;
        }

        if free_positions.len() > 0 && free_positions[0] < *key {
            positions.insert(free_positions[0], *value);
            positions.insert(*key, -1);
            free_positions.remove(0);
        }
    });

    positions
        .iter()
        .map(|(key, value)| {
            if *value == -1 {
                return 0;
            }
            *key as i64 * *value as i64
        })
        .sum::<i64>()
}

fn part2(digits: &Vec<u32>) -> i64 {
    let mut positions: HashMap<u32, i32> = HashMap::new();
    let mut file_positions: Vec<(u32, u32, u32)> = Vec::new(); // location, length, index
    let mut free_positions: Vec<(u32, u32)> = Vec::new(); // index, size

    let mut i = 0;

    digits.iter().enumerate().for_each(|(index, digit)| {
        if *digit == 0 {
            return;
        }

        if index % 2 == 0 {
            file_positions.insert(0, (i, *digit, (index / 2) as u32));
        } else {
            free_positions.push((i, *digit));
        }

        for _ in 0..*digit {
            if index % 2 == 0 {
                positions.insert(i, (index / 2) as i32);
            } else {
                positions.insert(i, -1);
            }
            i += 1;
        }
    });

    file_positions.iter().for_each(|(location, length, index)| {
        let mut free_index = 0;

        while free_index < free_positions.len() && free_positions[free_index].1 < *length {
            free_index += 1;
        }

        if free_index >= free_positions.len() {
            return;
        }

        let space_index = free_positions[free_index].0;

        if free_positions.len() == 0 || space_index > *location {
            return;
        }

        for i in 0..*length {
            positions.insert(space_index + i, *index as i32);
            positions.insert(location + i, -1);
        }

        free_positions[free_index].0 += length;
        free_positions[free_index].1 -= length;
    });

    positions
        .iter()
        .map(|(key, value)| {
            if *value == -1 {
                return 0;
            }
            *key as i64 * *value as i64
        })
        .sum::<i64>()
}
