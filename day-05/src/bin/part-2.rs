/// Day 5: If You Give A Seed A Fertilizer, Part 2
use std::ops::Range;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", get_min_location(input));
}

fn get_min_location(input: &str) -> i64 {
    println!("Started");
    let parts: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(parts.len(), 8);
    let seeds: Vec<i64> = parts[0]
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();
    let mut seed_ranges: Vec<Range<i64>> = Vec::new();
    for chunk in seeds.chunks(2) {
        let start = chunk[0];
        let length = chunk[1];
        seed_ranges.push(start..start + length);
    }
    println!("Seeds started");
    let maps: [Vec<(i64, i64, i64)>; 7] = create_maps(&parts[1..]);
    println!("Maps created");
    let mut minimum = -1;
    for seed_range in seed_ranges {
        for seed in seed_range {
            let location = get_seed_location(seed, &maps);
            if minimum == -1 || location < minimum {
                minimum = location;
            }
        }
    }
    minimum
}

fn get_seed_location(seed: i64, maps: &[Vec<(i64, i64, i64)>; 7]) -> i64 {
    let mut location = seed;
    for map in maps {
        for (target, start, length) in map {
            if location >= *start && location < *start + *length {
                let diff = target - start;
                location += diff;
                break;
            }
        }
    }
    location
}

fn create_maps(parts: &[&str]) -> [Vec<(i64, i64, i64)>; 7] {
    let mut maps: [Vec<(i64, i64, i64)>; 7] = Default::default();
    for map_number in 0..7 {
        for line in parts[map_number].lines().skip(1) {
            maps[map_number].push(line_mappings(line));
        }
    }
    maps
}

fn line_mappings(input: &str) -> (i64, i64, i64) {
    let numbers: Vec<i64> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    assert_eq!(numbers.len(), 3);

    (numbers[0], numbers[1], numbers[2])
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_line_ranges() {
        assert_eq!(line_mappings("0 1 2"), (0, 1, 2));
    }

    #[test]
    fn test_min_location() {
        assert_eq!(get_min_location(INPUT), 46);
    }
}
