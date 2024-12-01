fn main() {
    let input = include_str!("../part-1-input.txt");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        left.push(split[0].parse::<i32>().unwrap());
        right.push(split[1].parse::<i32>().unwrap());
    }
    println!("Part 1: {}", part_1(&mut left, &mut right));
    println!("Part 1: {}", part_2(&mut left, &mut right));
}

fn part_1(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    left.sort();
    right.sort();
    let mut diffs = Vec::new();

    for i in 0..right.len() {
        diffs.push((left[i] - right[i]).abs());
    }

    diffs.into_iter().sum()
}

fn part_2(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    left.sort();
    let mut similarity_scores = Vec::new();
    for number in left {
        similarity_scores.push(
            *number
                * right
                    .into_iter()
                    .filter(|element| *element == number)
                    .count() as i32,
        )
    }

    similarity_scores.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(part_1(&mut left, &mut right), 11);
    }

    #[test]
    fn test_part_2() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(part_2(&mut left, &mut right), 31);
    }
}
