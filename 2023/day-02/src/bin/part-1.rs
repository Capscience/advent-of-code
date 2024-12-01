/// Day 2: Cube Conundrum, Part 1

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

const LEADING_CHARACTERS: usize = 5;

fn main() {
    let input = include_str!("../../part-1-input.txt");
    let possible_games = input.lines().filter_map(|game| check_game(game));
    println!("{}", possible_games.sum::<u32>())
}

fn check_game(game: &str) -> Option<u32> {
    let game_parts: Vec<&str> = game.split(": ").collect();
    let game_id = game_parts[0][LEADING_CHARACTERS..].parse::<u32>().unwrap();
    let sets = game_parts[1].split("; ");
    for set in sets {
        if !check_set(set) {
            return None;
        }
    }
    Some(game_id)
}

fn check_set(set: &str) -> bool {
    let colors = set.split(", ").collect::<Vec<&str>>();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for color in colors {
        let parts: Vec<&str> = color.split(" ").collect();
        let count = parts[0];
        let color = parts[1];
        match color {
            "red" => red += count.parse::<u32>().unwrap(),
            "green" => green += count.parse::<u32>().unwrap(),
            "blue" => blue += count.parse::<u32>().unwrap(),
            _ => panic!("Unknown color"),
        }
    }
    red <= RED_CUBES && green <= GREEN_CUBES && blue <= BLUE_CUBES
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_game() {
        assert_eq!(
            check_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Some(1)
        );
        assert_eq!(
            check_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            Some(2)
        );
        assert_eq!(
            check_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            None
        );
        assert_eq!(
            check_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            None
        );
        assert_eq!(
            check_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            Some(5)
        );
    }
}
