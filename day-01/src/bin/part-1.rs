fn main() {
    let input = include_str!("../../part-1-input.txt");
    let mut numbers: Vec<i32> = Vec::new();
    for line in input.lines() {
        if let Ok(number) = decode_line(line) {
            numbers.push(number);
        } else {
            println!("Error decoding line: {}", line);
        }
    }
    let sum: i32 = numbers.iter().sum();
    println!("{}", sum);
}

fn decode_line(line: &str) -> Result<i32, String> {
    let mut line_digits = String::new();
    for char in line.chars() {
        if char.to_digit(10).is_some() {
            line_digits.push(char);
        }
    }
    if line_digits.len() == 0 {
        return Err(String::from("No digits found in line"));
    } else {
        let first_and_last = concat_first_and_last(&line_digits);
        return Ok(i32::from_str_radix(&first_and_last, 10).unwrap());
    }
}

/// Returns a String containing the first and last characters of the given string.
/// If the string is only one character long, the string is duplicated.
fn concat_first_and_last(string: &str) -> String {
    if string.len() == 1 {
        return string.to_string() + string;
    } else {
        let last_index = string.len() - 1;
        let mut first_and_last = String::new();
        first_and_last.push_str(&string[..1]);
        first_and_last.push_str(&string[last_index..]);
        return first_and_last;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_first_and_last() {
        assert_eq!(concat_first_and_last("123"), String::from("13"));
        assert_eq!(concat_first_and_last("1"), String::from("11"));
        assert_eq!(concat_first_and_last("12"), String::from("12"));
        assert_eq!(concat_first_and_last("1234"), String::from("14"));
    }

    #[test]
    fn test_decode_line() {
        assert_eq!(decode_line("1abc2"), Ok(12));
        assert_eq!(decode_line("pqr3stu8vwx"), Ok(38));
        assert_eq!(decode_line("a1b2c3d4e5f"), Ok(15));
        assert_eq!(decode_line("treb7uchet"), Ok(77));
        assert_eq!(decode_line("this should not work"), Err(String::from("No digits found in line")));
    }
}
