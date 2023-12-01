/// Day 1: Trebuchet?!, Part 2
use std::collections::HashMap;

fn main() {
    // Store the digit names in a HashMap for easy lookup.
    let digits: HashMap<&str, &str> = HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let input = include_str!("../../part-1-input.txt");
    let mut numbers: Vec<i32> = Vec::new();
    for line in input.lines() {
        let first_digit = get_first_digit(line, &digits);
        let last_digit = get_last_digit(line, &digits);
        if let (Some(first_digit), Some(last_digit)) = (first_digit, last_digit) {
            let first_and_last = first_digit + &last_digit;
            if let Ok(number) = i32::from_str_radix(&first_and_last, 10) {
                numbers.push(number);
            } else {
                println!("Error decoding line: {}", line);
            }
        } else {
            println!("Error decoding line: {}", line);
        }
    }
    let sum: i32 = numbers.iter().sum();
    println!("{}", sum);
}

fn get_first_digit(line: &str, digits: &HashMap<&str, &str>) -> Option<String> {
    for i in 0..line.len() {
        for (digit_name, digit) in digits {
            if line[i..].starts_with(digit_name) {
                return Some(digit.to_string());
            } else if line[i..].starts_with(digit) {
                return Some(digit.to_string());
            }
        }
    }
    None
}

fn get_last_digit(line: &str, digits: &HashMap<&str, &str>) -> Option<String> {
    for i in (0..line.len() + 1).rev() {
        for (digit_name, digit) in digits {
            if line[..i].ends_with(digit_name) {
                return Some(digit.to_string());
            } else if line[..i].ends_with(digit) {
                return Some(digit.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_digit() {
        let digits: HashMap<&str, &str> = HashMap::from([
            ("zero", "0"),
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ]);
        assert_eq!(
            get_first_digit("two1nine", &digits),
            Some(String::from("2"))
        );
        assert_eq!(
            get_first_digit("eightwothree", &digits),
            Some(String::from("8"))
        );
        assert_eq!(
            get_first_digit("abcone2threexyz", &digits),
            Some(String::from("1"))
        );
        assert_eq!(
            get_first_digit("xtwone3four", &digits),
            Some(String::from("2"))
        );
        assert_eq!(
            get_first_digit("4nineeightseven2", &digits),
            Some(String::from("4"))
        );
        assert_eq!(
            get_first_digit("zoneight234", &digits),
            Some(String::from("1"))
        );
        assert_eq!(
            get_first_digit("7pqrstsixteen", &digits),
            Some(String::from("7"))
        );
    }

    #[test]
    fn test_get_last_digit() {
        let digits: HashMap<&str, &str> = HashMap::from([
            ("zero", "0"),
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ]);
        assert_eq!(
            get_last_digit("two1nine", &digits),
            Some(String::from("9"))
        );
        assert_eq!(
            get_last_digit("eightwothree", &digits),
            Some(String::from("3"))
        );
        assert_eq!(
            get_last_digit("abcone2threexyz", &digits),
            Some(String::from("3"))
        );
        assert_eq!(
            get_last_digit("xtwone3four", &digits),
            Some(String::from("4"))
        );
        assert_eq!(
            get_last_digit("4nineeightseven2", &digits),
            Some(String::from("2"))
        );
        assert_eq!(
            get_last_digit("zoneight234", &digits),
            Some(String::from("4"))
        );
        assert_eq!(
            get_last_digit("7pqrstsixteen", &digits),
            Some(String::from("6"))
        );
    }
}
