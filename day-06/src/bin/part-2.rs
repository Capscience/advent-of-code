/// Day 6: Wait For It, Part 2

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", get_ways(input));
}

fn get_ways(input: &str) -> u64 {
    let input = input.replace(" ", "");
    let lines: Vec<&str> = input.lines().collect();
    let time: u64 = parse_line(lines[0]);
    let distance: u64 = parse_line(lines[1]);
    race_calculation(time, distance)
}

fn race_calculation(time: u64, distance: u64) -> u64 {
    let mut wins: u64 = 0;
    let mut current_time: u64 = 0;
    while current_time < time {
        if current_time * (time - current_time) > distance {
            wins += 1;
        }
        current_time += 1;
    }
    wins
}

fn parse_line(input: &str) -> u64 {
    input.split(":").nth(1).unwrap().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{get_ways, parse_line, race_calculation};

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parse_line() {
        let input = TEST_INPUT.replace(" ", "");
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(parse_line(lines[0]), 71530);
        assert_eq!(parse_line(lines[1]), 940200);
    }

    #[test]
    fn test_race_calculation() {
        assert_eq!(race_calculation(7, 9), 4);
        assert_eq!(race_calculation(15, 40), 8);
    }

    #[test]
    fn test_get_ways() {
        assert_eq!(get_ways(TEST_INPUT), 71503);
    }
}
