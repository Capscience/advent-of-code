use std::{fs, path::Path, time::Instant};

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Cannot solve without input!");

    let start_1 = Instant::now();
    println!("Part 1: {}, {:?}", part_1(&input), start_1.elapsed());
    let start_2 = Instant::now();
    println!("Part 2: {}, {:?}", part_2(&input), start_2.elapsed());
}

fn part_1(input: &str) {
    todo!();
}

fn part_2(input: &str) {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        todo!();
    }
}
