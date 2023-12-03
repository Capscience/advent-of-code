/// Day 3: Gear Ratios, Part 2

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", sum_gear_ratios(input));
}

/// Sums all gear ratios. Gears are '*'-symbols that are adjacent to exactly 2 numbers.
/// Gear ratios are the product of the two adjacent numbers.
fn sum_gear_ratios(input: &str) -> u32 {
    let mut sum = 0;
    for (index, char) in input.chars().enumerate() {
        if char == '*' {
            if let Some(ratio) = gear_ratio(&input, index) {
                sum += ratio;
            }
        }
    }
    sum
}

/// Returns gear ratio of gear at given index. Returns None if gear is not adjacent to exactly 2 numbers.
fn gear_ratio(input: &str, index: usize) -> Option<u32> {
    let line_length = input.lines().next().unwrap().len() + 1;
    let line_count = input.lines().count();

    let checkable = get_checkable_sides(index, line_length, line_count); // [topleft, top, topright, left, right, bottomleft, bottom, bottomright]

    let mut adjacent_number_indexes = Vec::new();

    for (side, checkable) in checkable.iter().enumerate() {
        if !checkable {
            continue;
        }
        let check_index = match side {
            0 => index - line_length - 1,
            1 => index - line_length,
            2 => index - line_length + 1,
            3 => index - 1,
            4 => index + 1,
            5 => index + line_length - 1,
            6 => index + line_length,
            7 => index + line_length + 1,
            _ => panic!("Invalid side"),
        };
        let char = input.chars().nth(check_index).unwrap();
        if char.is_digit(10) {
            adjacent_number_indexes.push(number_index_range(input, check_index));
        }
    }
    adjacent_number_indexes.dedup();

    if adjacent_number_indexes.len() == 2 {
        let numbers = adjacent_number_indexes
            .iter()
            .map(|(start, end)| input[*start..=*end].parse::<u32>().unwrap());
        Some(numbers.product())
    } else {
        None
    }
}

/// Returns the start and end index of the number at given index
fn number_index_range(input: &str, index: usize) -> (usize, usize) {
    let line_length = input.lines().next().unwrap().len() + 1;
    let mut start = index;
    let mut end = index;

    while start % line_length != 0 && input.chars().nth(start - 1).unwrap().is_digit(10) {
        start -= 1;
    }
    while end % line_length != line_length - 2 && input.chars().nth(end + 1).unwrap().is_digit(10) {
        end += 1;
    }
    (start, end)
}

/// Returns which sides of a gear at given index can be checked without index out of bounds
fn get_checkable_sides(index: usize, line_length: usize, line_count: usize) -> [bool; 8] {
    let mut checkable = [true; 8]; // [topleft, top, topright, left, right, bottomleft, bottom, bottomright]

    // Check which directions can be checked
    if index < line_length {
        checkable[0] = false;
        checkable[1] = false;
        checkable[2] = false;
    }
    if index > line_length * (line_count - 1) {
        checkable[5] = false;
        checkable[6] = false;
        checkable[7] = false;
    }
    if index % line_length == 0 {
        checkable[0] = false;
        checkable[3] = false;
        checkable[5] = false;
    }
    if index % line_length == line_length - 1 {
        checkable[2] = false;
        checkable[4] = false;
        checkable[7] = false;
    }
    checkable
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
.......755
...$..*...
.664.598..";

    #[test]
    fn test_gear_ratio() {
        assert_eq!(gear_ratio(INPUT, 0), None);
        assert_eq!(gear_ratio(INPUT, 94), Some(755 * 598));
    }

    #[test]
    fn test_sum_gear_ratios() {
        assert_eq!(sum_gear_ratios(INPUT), 467835);
    }
}
