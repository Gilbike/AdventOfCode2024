use std::fs;

fn main() {
    let lines = get_input(false);

    println!("Part 1 Result: {}", part1(&lines));
    println!("Part 2 Result: {}", part2(&lines));
}

fn part1(lines: &Vec<Vec<i32>>) -> usize {
    lines.iter().filter_map(|line| is_line_valid(line)).count()
}

fn part2(lines: &Vec<Vec<i32>>) -> usize {
    lines
        .iter()
        .filter(|line| {
            line
                .iter()
                .enumerate()
                .any(|(index, _)| is_line_valid(&[&line[..index], &line[index + 1..]].concat()).is_some())
        })
        .count()
}

fn is_line_valid(line: &Vec<i32>) -> Option<()> {
    let sorted = line.windows(2).all(|numbers| numbers[0] < numbers[1]);
    let sorted_reverse = line.windows(2).all(|numbers| numbers[0] > numbers[1]);

    if !sorted && !sorted_reverse {
        return None;
    }

    if !line.windows(2).all(|numbers| (numbers[0] - numbers[1]).abs() >= 1 && (numbers[0] - numbers[1]).abs() <= 3) {
        return None;
    }

    Some(())
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

        let result = part1(&lines);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let lines = get_input(true);

        let result = part2(&lines);
        assert_eq!(result, 4);
    }
}