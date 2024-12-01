/// Day 2: Cube Conundrum, Part 2

fn main() {
    let input = include_str!("../../part-1-input.txt");
    let game_powers = input.lines().map(|game| game_power(game));
    println!("{}", game_powers.sum::<u32>())
}

fn game_power(game: &str) -> u32 {
    let game_parts: Vec<&str> = game.split(": ").collect();
    let sets = game_parts[1].split("; ");
    let colors = sets.map(|set| get_colors(set));
    let red = colors.clone().map(|(r, _, _)| r).max().unwrap();
    let green = colors.clone().map(|(_, g, _)| g).max().unwrap();
    let blue = colors.clone().map(|(_, _, b)| b).max().unwrap();
    red * green * blue
}

fn get_colors(set: &str) -> (u32, u32, u32) {
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
    (red, green, blue)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_game_power() {
        assert_eq!(
            game_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );
        assert_eq!(
            game_power("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            12
        );
        assert_eq!(
            game_power("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            1560
        );
        assert_eq!(
            game_power("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            630
        );
        assert_eq!(
            game_power("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            36
        );
    }
}
