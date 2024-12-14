use std::{collections::HashMap, fs, path::Path, time::Instant};

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path)
        .expect("Cannot solve without input!")
        .split_whitespace()
        .map(|n| n.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    let start_1 = Instant::now();
    println!("Part 1: {}, {:?}", solve(&input, 25), start_1.elapsed());
    let start_2 = Instant::now();
    println!("Part 2: {}, {:?}", solve(&input, 75), start_2.elapsed());
    let start_3 = Instant::now();
    println!(
        "Part 3 (just for fun): {}, {:?}",
        solve(&input, 207),
        start_3.elapsed()
    );
}

fn solve(input: &[u128], depth: u8) -> u128 {
    let mut cache: HashMap<(u128, u8), u128> = HashMap::new();
    let mut length = 0;
    for stone in input {
        length += blink(*stone, depth, &mut cache);
    }
    length
}

fn blink(stone: u128, depth: u8, cache: &mut HashMap<(u128, u8), u128>) -> u128 {
    if depth == 0 {
        return 1;
    }
    if let Some(len) = cache.get(&(stone, depth)) {
        return *len;
    }
    let result = if stone == 0 {
        blink(1, depth - 1, cache)
    } else if (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
        let number_string = stone.to_string();
        let (a, b) = number_string.split_at(number_string.len() / 2);
        blink(a.parse().unwrap(), depth - 1, cache) + blink(b.parse().unwrap(), depth - 1, cache)
    } else {
        blink(stone * 2024, depth - 1, cache)
    };
    cache.insert((stone, depth), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let stones = vec![125, 17];
        assert_eq!(solve(&stones, 25), 55312);
    }
}
