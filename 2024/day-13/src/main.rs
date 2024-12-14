use std::{fs, path::Path, time::Instant};

fn main() {
    let path = Path::new("input.txt");
    let input: Vec<Machine> = fs::read_to_string(path)
        .expect("Cannot solve without input!")
        .split("\n\n")
        .map(|machine| Machine::new(machine))
        .collect();

    let start_1 = Instant::now();
    println!("Part 1: {}, {:?}", part_1(&input), start_1.elapsed());
    let start_2 = Instant::now();
    println!("Part 2: {}, {:?}", part_2(&input), start_2.elapsed());
}

fn part_1(input: &[Machine]) -> i64 {
    let mut sum: i64 = 0;
    for machine in input {
        if let Some(solution) = machine.solve(0) {
            sum += solution;
        }
    }
    sum
}

fn part_2(input: &[Machine]) -> i64 {
    let mut sum: i64 = 0;
    for machine in input {
        if let Some(solution) = machine.solve(10000000000000) {
            sum += solution;
        }
    }
    sum
}

#[derive(Debug)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn new(input: &str) -> Self {
        let mut values = input.lines().map(|line| {
            line.split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|part| part[2..].parse::<i64>().unwrap())
        });
        let mut button_a_values = values.next().unwrap();
        let mut button_b_values = values.next().unwrap();
        let mut prize_values = values.next().unwrap();
        Self {
            button_a: (
                button_a_values.next().unwrap(),
                button_a_values.next().unwrap(),
            ),
            button_b: (
                button_b_values.next().unwrap(),
                button_b_values.next().unwrap(),
            ),
            prize: (prize_values.next().unwrap(), prize_values.next().unwrap()),
        }
    }

    fn solve(&self, prize_modifier: i64) -> Option<i64> {
        // equation format: eq.0 * a + eq.1 * b = eq.2
        let mut eq_1 = [
            self.button_a.0,
            self.button_b.0,
            self.prize.0 + prize_modifier,
        ];
        let mut eq_2 = [
            self.button_a.1,
            self.button_b.1,
            self.prize.1 + prize_modifier,
        ];

        // Multiply so that a-term has same value
        let eq_2_multiplier = eq_1[0];
        let eq_1_multiplier = eq_2[0];
        for item in &mut eq_1 {
            *item *= eq_1_multiplier;
        }
        for item in &mut eq_2 {
            *item *= eq_2_multiplier;
        }

        // Substitute a to find b
        let b_eq = (eq_1[1] - eq_2[1], eq_1[2] - eq_2[2]);
        if b_eq.1 % b_eq.0 != 0 {
            return None;
        }
        let b = b_eq.1 / b_eq.0;

        // Substitute b to find a
        let a_eq = (eq_2[0], eq_2[2] - (b * eq_2[1]));
        if a_eq.1 % a_eq.0 != 0 {
            return None;
        }
        let a = a_eq.1 / a_eq.0;
        Some(a * 3 + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_solve() {
        let machine = Machine::new(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400",
        );
        assert_eq!(machine.solve(0), Some(280));
        let machine = Machine::new(
            "Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176",
        );
        assert_eq!(machine.solve(0), None);
        let machine = Machine::new(
            "Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450",
        );
        assert_eq!(machine.solve(0), Some(200));
        let machine = Machine::new(
            "Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        );
        assert_eq!(machine.solve(0), None);
    }

    #[test]
    fn temp_test() {
        let machine = Machine::new(
            "Button A: X+2, Y+3
Button B: X+2, Y+1
Prize: X=8, Y=6",
        );
        assert_eq!(machine.solve(0), Some(6));
    }
}
