use regex::Regex;
use std::{fs, path::Path};

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path)
        .expect("Input file not found!")
        .replace("\n", "");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let mut result: u32 = 0;
    let pattern = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for (_, [a, b]) in pattern.captures_iter(&input).map(|c| c.extract()) {
        result += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
    }
    result
}

fn part_2(input: &str) -> u32 {
    let mut result: u32 = 0;
    let pattern = Regex::new(r"(^.*?don't\(\)|do\(\).*?don't\(\)|do\(\).*$)").unwrap();
    for do_block in pattern.find_iter(input).map(|c| c.as_str()) {
        result += part_1(do_block);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
