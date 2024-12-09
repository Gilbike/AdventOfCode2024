use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("./day1.input")
        .expect("Should have been able to read the file");

    let (left, right) = parse_input(contents);

    let result1 = part1(&left, &right);
    let result2 = part2(left, right);

    println!("Result part 1: {}", result1);
    println!("Result part 2: {}", result2);
}

fn parse_input(input: String) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .filter_map(|line| line.split_once("   ")) 
        .map(|(left, right)| {
            (
                left.parse::<i32>().unwrap(),
                right.parse::<i32>().unwrap(),
            )
        })
        .unzip()
}

pub fn part1(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    /* ChatGPT ideal
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
    */

    let mut cloned_left = left.clone();
    let mut cloned_right = right.clone();

    cloned_left.sort();
    cloned_right.sort();

    let mut differences: Vec<i32> = vec![];

    for i in 0..left.len() {
        differences.push((cloned_left[i] - cloned_right[i]).abs());
    }

    differences.iter().sum()
}

pub fn part2(left: Vec<i32>, right: Vec<i32>) -> i32 {
    /* ~ ideal solution */

    let mut appereances = HashMap::new();

    left.iter().for_each(|number| {
        appereances.insert(*number, 0);
    });

    right.iter().for_each(|number| {
        appereances.entry(*number).and_modify(|counter| *counter += 1);
    });

    appereances.into_iter().map(|(k, v)| k * v).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let contents = fs::read_to_string("./day1.test")
            .expect("Should have been able to read the file");

        let (left, right) = parse_input(contents);

        let result = part1(&left, &right);
        assert_eq!(result, 11);
    }

    #[test]
    fn part2_works() {
        let contents = fs::read_to_string("./day1.test")
            .expect("Should have been able to read the file");

        let (left, right) = parse_input(contents);

        let result = part2(left, right);
        assert_eq!(result, 11);
    }
}