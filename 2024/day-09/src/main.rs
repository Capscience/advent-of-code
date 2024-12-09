use std::{fs, path::Path, time::Instant};

fn main() {
    let path = Path::new("input.txt");
    let input: Vec<u8> = fs::read_to_string(path)
        .expect("Cannot solve without input!")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let start_1 = Instant::now();
    println!("Part 1: {}, {:?}", "part_1()", start_1.elapsed());
    let start_2 = Instant::now();
    println!("Part 2: {}, {:?}", "part_2()", start_2.elapsed());
}

fn part_1(input: &Vec<u8>) -> usize {
    eprintln!("Start part 1");
    let mut checksum: usize = 0;

    let mut blocks_reversed = input.iter().step_by(2).enumerate().rev();
    let (mut last_block_id, mut last_block_len) = blocks_reversed.next().unwrap();
    let mut last_index: usize = input.iter().map(|n| *n as usize).sum();
    last_index -= 1; // Prevent index out of range
    let mut disk_index: usize = 0;
    let mut main_iterator = input.windows(2).step_by(2).enumerate();

    eprintln!("Start while let");
    while let Some((block_id, [block_len, empty_len])) = main_iterator.next() {
        let mut next_index = disk_index + *block_len as usize;
        for i in disk_index..next_index {
            dbg!(i, block_id);
            checksum += i * block_id;
        }
        disk_index = next_index;
        next_index = disk_index + *empty_len as usize;
        for i in disk_index..next_index {
            if last_index <= i {
                return checksum;
            }
            if *last_block_len == 0 {
                (last_block_id, last_block_len) = blocks_reversed.next().unwrap();
            }

            dbg!(i, last_block_id);
            checksum += i * last_block_id;
            last_index -= 1;
        }
        disk_index = next_index;
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![1, 2, 3, 4, 5];
        assert_eq!(part_1(&input), 61);
    }
}
