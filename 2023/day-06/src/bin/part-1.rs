/// Day 6: Wait For It, Part 1

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", get_ways(input));
}

fn get_ways(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<u32> = parse_line(lines[0]);
    let distances: Vec<u32> = parse_line(lines[1]);
    let mut ways: u32 = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        ways *= race_calculation(*time, *distance);
    }
    ways
}

fn race_calculation(time: u32, distance: u32) -> u32 {
    let mut wins: u32 = 0;
    let mut current_time: u32 = 0;
    while current_time < time {
        if current_time * (time - current_time) > distance {
            wins += 1;
        }
        current_time += 1;
    }
    wins
}

fn parse_line(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{get_ways, parse_line, race_calculation};

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parse_line() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        assert_eq!(parse_line(lines[0]), vec![7, 15, 30]);
        assert_eq!(parse_line(lines[1]), vec![9, 40, 200]);
    }

    #[test]
    fn test_race_calculation() {
        assert_eq!(race_calculation(7, 9), 4);
        assert_eq!(race_calculation(15, 40), 8);
    }

    #[test]
    fn test_get_ways() {
        assert_eq!(get_ways(TEST_INPUT), 288);
    }
}
