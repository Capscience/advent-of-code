use std::fs;

enum Variant {
    NotKnown,
    Increasing,
    Decreasing,
}

fn main() {
    let path = "input.txt".to_string();
    let input = fs::read_to_string(path).expect("No input file found!");

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().expect("Not an integer!"))
                .collect()
        })
        .collect();

    println!("Part 1: {}", part_1(&reports));
    println!("Part 2: {}", part_2(reports));
}

fn part_1(reports: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;

    for report in reports {
        if is_safe(report.iter()) {
            total += 1;
        }
    }

    total
}

fn part_2(reports: Vec<Vec<i32>>) -> i32 {
    let mut total = 0;

    for report in reports {
        if is_safe(report.iter()) {
            total += 1;
        } else if is_safe_dampened(report) {
            total += 1
        }
    }

    total
}

fn is_safe<'a, T: Iterator<Item = &'a i32>>(report: T) -> bool {
    use std::cmp::Ordering::*;
    use Variant::*;

    let mut previous: Option<i32> = None;
    let mut variant = NotKnown;

    for number in report {
        if let Some(value) = previous {
            if matches!(variant, NotKnown) {
                match number.cmp(&value) {
                    Less => variant = Decreasing,
                    Greater => variant = Increasing,
                    Equal => {
                        return false;
                    }
                }
            }

            match variant {
                NotKnown => unreachable!("NotKnown handled separately."),
                Increasing => {
                    if *number <= value || number - value > 3 {
                        return false;
                    }
                }
                Decreasing => {
                    if *number >= value || value - number > 3 {
                        return false;
                    }
                }
            }
        }
        previous = Some(*number);
    }
    true
}

fn is_safe_dampened(report: Vec<i32>) -> bool {
    for i in 0..report.len() {
        if is_safe(
            report
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != i)
                .map(|(_, value)| value),
        ) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        assert!(is_safe(vec![7, 6, 4, 2, 1].iter()));
        assert!(!is_safe(vec![1, 2, 7, 8, 9].iter()));
        assert!(!is_safe(vec![9, 7, 6, 2, 1].iter()));
        assert!(!is_safe(vec![1, 3, 2, 4, 5].iter()));
        assert!(!is_safe(vec![8, 6, 4, 4, 1].iter()));
        assert!(is_safe(vec![1, 3, 6, 7, 9].iter()));
    }

    #[test]
    fn test_is_safe_dampened() {
        assert!(is_safe_dampened(vec![7, 6, 4, 2, 1]));
        assert!(!is_safe_dampened(vec![1, 2, 7, 8, 9]));
        assert!(!is_safe_dampened(vec![9, 7, 6, 2, 1]));
        assert!(is_safe_dampened(vec![1, 3, 2, 4, 5]));
        assert!(is_safe_dampened(vec![8, 6, 4, 4, 1]));
        assert!(is_safe_dampened(vec![1, 3, 6, 7, 9]));
    }
}
