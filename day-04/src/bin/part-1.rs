/// Day 4: Scratchcards, Part 1

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", sum_scratchcards(input));
}

fn sum_scratchcards(input: &str) -> u32 {
    let mut sum = 0;
    for card in input.lines() {
        sum += scratchcard_points(card);
    }
    sum
}

/// Calculate the points for a single scratchcard.
/// Assumes that each winning number and each your number are unique.
fn scratchcard_points(card: &str) -> u32 {
    let all_numbers_str: &str = card.split(": ").nth(1).unwrap();
    let mut all_numbers = all_numbers_str
        .replace(" | ", " ")
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let len_all = all_numbers.len();
    all_numbers.sort_unstable(); // dedup only removes consecutive duplicates, so sort first
    all_numbers.dedup();
    let len_deduped = all_numbers.len();

    // how many duplicates, i.e. numbers in both winning and your numbers
    let winning_numbers = len_all - len_deduped;

    if winning_numbers == 0 {
        return 0;
    } else {
        return 1_u32 << (winning_numbers - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_sum_scratchcard_points() {
        assert_eq!(sum_scratchcards(INPUT), 13);
    }

    #[test]
    fn test_scratchcard_points() {
        assert_eq!(
            scratchcard_points("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            8
        );
        assert_eq!(
            scratchcard_points("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            2
        );
        assert_eq!(
            scratchcard_points("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            2
        );
        assert_eq!(
            scratchcard_points("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            1
        );
        assert_eq!(
            scratchcard_points("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
        assert_eq!(
            scratchcard_points("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            0
        );
    }
}
