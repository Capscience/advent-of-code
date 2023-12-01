fn main() {
    let input = include_str!("../../part-1-input.txt");
    let mut numbers: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut line_digits = String::new();
        for char in line.chars() {
            if char.to_digit(10).is_some() {
                line_digits.push(char);
            }
        }
        if line_digits.len() == 1 {
            numbers.push(i32::from_str_radix(&line_digits, 10).unwrap() * 11);
        } else {
            let last_index = line_digits.len() - 1;
            let mut first_and_last = String::new();
            first_and_last.push_str(&line_digits[0..1]);
            first_and_last.push_str(&line_digits[last_index..]);
            numbers.push(i32::from_str_radix(&first_and_last, 10).unwrap());
        }
    }
    let sum: i32 = numbers.iter().sum();
    println!("{}", sum);
}
