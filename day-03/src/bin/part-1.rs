/// Day 3: Gear Ratios, Part 1

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", sum_partnumbers(input));
}

/// Sums all partnumbers. Partnumbers are numbers, that are adjacent to a symbol.
fn sum_partnumbers(input: &str) -> u32 {
    let mut sum = 0;
    let line_length = input.lines().next().unwrap().len();
    let line_count = input.lines().count();
    let mut current_number = String::new();
    let mut current_is_partnumber = false;
    let input = input.replace('\n', "");

    for (i, char) in input.chars().enumerate() {
        if char.is_digit(10) {
            current_number.push(char);
            current_is_partnumber =
                next_to_symbol(&input, i, line_length, line_count) || current_is_partnumber;
            if i % line_length == line_length - 1 {
                // End of line
                if current_is_partnumber {
                    sum += current_number.parse::<u32>().unwrap();
                }
                current_number = String::new();
                current_is_partnumber = false;
            }
        } else {
            // Number ended
            if current_is_partnumber {
                sum += current_number.parse::<u32>().unwrap();
            }
            current_number = String::new();
            current_is_partnumber = false;
        }
    }
    sum
}

/// Returns true if character in given index is adjacent to a symbol
fn next_to_symbol(input: &str, index: usize, line_length: usize, line_count: usize) -> bool {
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

    for (side, checkable) in checkable.iter().enumerate() {
        if !checkable {
            continue;
        }
        // Checkable, check if next to symbol
        if match side {
            0 => is_symbol(input, index - line_length - 1),
            1 => is_symbol(input, index - line_length),
            2 => is_symbol(input, index - line_length + 1),
            3 => is_symbol(input, index - 1),
            4 => is_symbol(input, index + 1),
            5 => is_symbol(input, index + line_length - 1),
            6 => is_symbol(input, index + line_length),
            7 => is_symbol(input, index + line_length + 1),
            _ => {
                panic!("Invalid side: {}", side)
            }
        } {
            return true;
        }
    }
    false
}

/// Returns true if character in given index is a is_symbol
/// ie. not a number or a '.'
fn is_symbol(input: &str, index: usize) -> bool {
    let char = input.chars().nth(index);
    if let Some(char) = char {
        if !(char.is_digit(10) || char == '.') {
            return true;
        }
    }
    false
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
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_1() {
        assert_eq!(sum_partnumbers(INPUT), 4361);
    }

    #[test]
    fn test_next_to_symbol() {
        let input = &INPUT.replace('\n', "");
        assert_eq!(next_to_symbol(input, 0, 10, 10), false);
        assert_eq!(next_to_symbol(input, 2, 10, 10), true);
    }

    #[test]
    fn test_is_symbol() {
        let input = &INPUT.replace('\n', "");
        assert_eq!(is_symbol(input, 0), false);
        assert_eq!(is_symbol(input, 3), false);
        assert_eq!(is_symbol(input, 13), true);
        assert_eq!(is_symbol(input, 36), true);
    }
}
