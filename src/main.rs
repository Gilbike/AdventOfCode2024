mod input;

use std::collections::HashMap;

use input::get_input;

use crate::input::get_test_input;

fn main() {
    let test_input = get_test_input();
    let input = get_input();

    println!("Part 1 Test Result: {}", part1(&test_input));
    println!("Part 1 Result: {}", part1(&input));
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
