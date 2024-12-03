use std::fs;

fn main() {
    let lines = get_input(false);

    println!("Part 1 Result: {}", part1(lines));
}

fn part1(lines: Vec<Vec<i32>>) -> i32 {
    lines.iter().map(|line| {
        let mut comparer = line.clone();
        comparer.sort();

        let mut comparer_rev = line.clone();
        comparer_rev.sort();
        comparer_rev.reverse();

        if *line != comparer && *line != comparer_rev {
            return 0;
        }

        for i in 0..comparer_rev.len()-1 {
            let diff = comparer_rev[i] - comparer_rev[i + 1];
            if diff > 3 || diff < 1 {
                return 0;
            }
        }

        1
    }).sum()
}

fn get_input(test: bool) -> Vec<Vec<i32>> {
    let path = if test { "test" } else { "input" };

    let contents = fs::read_to_string(format!("./.{}", path))
        .expect("Should have been able to read the file");

    contents
        .lines()
        .map(|line| line.split(' ').map(|number| number.parse::<i32>().unwrap()).collect()) 
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let lines = get_input(true);

        // let result = part1(&left, &right);
        // assert_eq!(result, 11);
    }
}