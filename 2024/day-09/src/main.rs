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
    println!("Part 1: {}, {:?}", part_1(&input), start_1.elapsed());
    let start_2 = Instant::now();
    println!("Part 2: {}, {:?}", part_2(&input), start_2.elapsed());
}

fn part_1(input: &Vec<u8>) -> usize {
    let mut disk = get_disk(input);
    let mut ptr = 0;
    let mut tail_ptr = disk.len() - 1;
    let mut checksum = 0;

    while ptr <= tail_ptr {
        if let Some(id) = disk[ptr] {
            checksum += ptr * id;
            ptr += 1;
        } else {
            disk.swap(ptr, tail_ptr);
            checksum += ptr * disk[ptr].unwrap();
            ptr += 1;
            while disk[tail_ptr].is_none() && ptr <= tail_ptr {
                tail_ptr -= 1;
            }
        }
    }
    checksum
}

fn part_2(input: &Vec<u8>) -> usize {
    let mut disk = get_disk(input);
    let mut tail_ptr = disk.len() - 1;
    let mut current_file = disk[tail_ptr];
    let mut current_file_len = 0;

    while 0 < tail_ptr {
        // Find current file location and length
        while disk[tail_ptr] == current_file && tail_ptr > 0 {
            tail_ptr -= 1;
            current_file_len += 1;
        }

        // Search for a spot
        let mut first_none = None;
        let mut ptr = 0;
        while ptr <= tail_ptr {
            if disk[ptr].is_none() {
                if first_none.is_none() {
                    first_none = Some(ptr);
                }
                // Found a spot, move the whole file
                if ptr + 1 - first_none.unwrap() >= current_file_len {
                    for i in 0..current_file_len {
                        disk.swap(first_none.unwrap() + i, tail_ptr + 1 + i);
                    }
                    break;
                }
            } else {
                first_none = None;
            }
            ptr += 1;
        }

        // Find next file
        while disk[tail_ptr].is_none() && tail_ptr > 0 {
            tail_ptr -= 1;
        }
        current_file = disk[tail_ptr];
        current_file_len = 0;
    }
    checksum(&disk)
}

fn checksum(disk: &[Option<usize>]) -> usize {
    let mut checksum = 0;
    for (i, block) in disk.iter().enumerate() {
        if let Some(id) = block {
            checksum += id * i;
        }
    }
    checksum
}

fn get_disk(input: &Vec<u8>) -> Vec<Option<usize>> {
    let mut disk: Vec<Option<usize>> = Vec::new();
    for (id, block) in input.iter().enumerate() {
        for _ in 0..*block {
            if id % 2 == 0 {
                disk.push(Some(id / 2));
            } else {
                disk.push(None);
            }
        }
    }
    disk
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![1, 2, 3, 4, 5];
        assert_eq!(part_1(&input), 60);
        let input: Vec<u8> = "2333133121414131402"
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        assert_eq!(part_1(&input), 1928);
    }

    #[test]
    fn test_part_2() {
        let input: Vec<u8> = "2333133121414131402"
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        assert_eq!(part_2(&input), 2858);
    }
}
