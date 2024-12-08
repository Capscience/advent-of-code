use std::{fs, path::Path};

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Cannot solve without input!");

    let parsed_input: Vec<(u64, Vec<u64>)> = input.lines().map(parse_line).collect();

    println!("Part 1: {}", part_1(&parsed_input));
    println!("Part 2: {}", part_2(&parsed_input));
}

fn part_1(input: &[(u64, Vec<u64>)]) -> u64 {
    let mut sum = 0;
    for equation in input {
        if is_valid1(equation) {
            sum += equation.0;
        }
    }
    sum
}

fn part_2(input: &[(u64, Vec<u64>)]) -> u64 {
    let mut sum = 0;
    for row in input {
        if is_valid2(row) {
            sum += row.0;
        }
    }
    sum
}

fn is_valid1(equation: &(u64, Vec<u64>)) -> bool {
    let mut values = vec![equation.1[0]];
    for number in equation.1.iter().skip(1) {
        values = check1(*number, values);
    }
    values.contains(&equation.0)
}

fn check1(number: u64, previous: Vec<u64>) -> Vec<u64> {
    let mut values = Vec::new();
    for value in previous {
        values.push(number + value);
        values.push(number * value);
    }
    values
}

fn is_valid2(equation: &(u64, Vec<u64>)) -> bool {
    let mut values = vec![equation.1[0]];
    for number in equation.1.iter().skip(1) {
        values = check2(*number, values);
    }
    values.contains(&equation.0)
}

fn check2(number: u64, previous: Vec<u64>) -> Vec<u64> {
    let mut values = Vec::new();
    for value in previous {
        values.push(number + value);
        values.push(number * value);
        values.push(concatenation(value, number));
    }
    values
}

fn concatenation(left: u64, right: u64) -> u64 {
    format!("{left}{right}").parse().unwrap()
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let mut parts = line.split(": ");
    let total = parts.next().unwrap().parse().unwrap();
    let components = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();
    (total, components)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("190: 10 19"), (190, vec![10, 19]));
    }

    #[test]
    fn test_concat() {
        assert_eq!(concatenation(15, 6), 156);
    }

    #[test]
    fn test_is_valid1() {
        assert!(is_valid1(&(190, vec![10, 19])));
        assert!(is_valid1(&(292, vec![11, 6, 16, 20])));
        assert!(!is_valid1(&(21037, vec![9, 7, 18, 13])))
    }

    #[test]
    fn test_is_valid2() {
        assert!(is_valid2(&(190, vec![10, 19])));
        assert!(is_valid2(&(156, vec![15, 6])));
        assert!(is_valid2(&(292, vec![11, 6, 16, 20])));
        assert!(!is_valid2(&(21037, vec![9, 7, 18, 13])))
    }

    #[test]
    fn test_part_1() {
        let parsed_input: Vec<(u64, Vec<u64>)> = INPUT.lines().map(parse_line).collect();
        assert_eq!(part_1(&parsed_input), 3749);
    }

    #[test]
    fn test_part_2() {
        let parsed_input: Vec<(u64, Vec<u64>)> = INPUT.lines().map(parse_line).collect();
        assert_eq!(part_2(&parsed_input), 11387);
    }
}
