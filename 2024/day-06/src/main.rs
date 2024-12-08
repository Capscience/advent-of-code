use core::panic;
use std::{fs, path::Path, usize};

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Cannot solve without input!");

    println!("Part 1: {}", part_1(Map::new(&input)));
    println!("Part 2: {}", part_2(Map::new(&input)));
}

fn part_1(mut map: Map) -> u32 {
    let mut count = 0;
    let mut visited: Vec<(usize, usize)> = Vec::new();

    loop {
        if !visited.contains(&(map.guard_i, map.guard_j)) {
            visited.push((map.guard_i, map.guard_j));
            count += 1;
        }
        if map.tick() {
            return count;
        }
    }
}

fn part_2(mut map: Map) -> u32 {
    let mut visited: Vec<(usize, usize)> = Vec::new();

    loop {
        if !visited.contains(&(map.guard_i, map.guard_j)) {
            visited.push((map.guard_i, map.guard_j));
        }
        if map.tick() {
            break;
        }
    }

    let mut count = 0;
    for (i, j) in visited {
        map.obstacles[i][j] = true;
        if is_loop(&mut map) {
            count += 1;
        }
        map.obstacles[i][j] = false;
    }
    count
}

#[derive(Debug)]
struct Map {
    obstacles: Vec<Vec<bool>>,
    guard_i: usize,
    guard_j: usize,
    guard_direction: Direction,
    initial_guard_i: usize,
    initial_guard_j: usize,
    initial_guard_direction: Direction,
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }
}

impl Map {
    fn new(input: &str) -> Self {
        use Direction::*;
        let mut obstacles = Vec::new();
        let (mut guard_i, mut guard_j) = (0, 0);
        let mut guard_direction = Up;

        for (i, line) in input.lines().enumerate() {
            obstacles.push(
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '.' => false,
                        '#' => true,
                        '^' => {
                            (guard_i, guard_j) = (i, j);
                            false
                        }
                        '>' => {
                            (guard_i, guard_j) = (i, j);
                            guard_direction = Right;
                            false
                        }
                        '<' => {
                            (guard_i, guard_j) = (i, j);
                            guard_direction = Left;
                            false
                        }
                        'v' => {
                            (guard_i, guard_j) = (i, j);
                            guard_direction = Left;
                            false
                        }
                        _ => panic!("Invalid input format!"),
                    })
                    .collect(),
            )
        }
        Self {
            obstacles,
            guard_i,
            guard_j,
            guard_direction: guard_direction.clone(),
            initial_guard_i: guard_i,
            initial_guard_j: guard_j,
            initial_guard_direction: guard_direction,
        }
    }

    fn tick(&mut self) -> bool {
        if let Some(next_position) =
            self.next(self.guard_i, self.guard_j, self.guard_direction.clone())
        {
            (self.guard_i, self.guard_j, self.guard_direction) = next_position;
            false
        } else {
            true
        }
    }

    fn next(
        &self,
        i: usize,
        j: usize,
        mut direction: Direction,
    ) -> Option<(usize, usize, Direction)> {
        use Direction::*;
        match direction {
            Up => {
                if i > 0 {
                    if self.obstacles[i - 1][j] {
                        direction = direction.turn();
                        Some((i, j, direction))
                    } else {
                        Some((i - 1, j, direction))
                    }
                } else {
                    None
                }
            }
            Down => {
                if i < self.obstacles.len() - 1 {
                    if self.obstacles[i + 1][j] {
                        direction = direction.turn();
                        Some((i, j, direction))
                    } else {
                        Some((i + 1, j, direction))
                    }
                } else {
                    None
                }
            }
            Right => {
                if j < self.obstacles.len() - 1 {
                    if self.obstacles[i][j + 1] {
                        direction = direction.turn();
                        Some((i, j, direction))
                    } else {
                        Some((i, j + 1, direction))
                    }
                } else {
                    None
                }
            }
            Left => {
                if j > 0 {
                    if self.obstacles[i][j - 1] {
                        direction = direction.turn();
                        Some((i, j, direction))
                    } else {
                        Some((i, j - 1, direction))
                    }
                } else {
                    None
                }
            }
        }
    }
}

fn is_loop(map: &mut Map) -> bool {
    let mut visited: Vec<(usize, usize, Direction)> = Vec::new();
    map.guard_i = map.initial_guard_i;
    map.guard_j = map.initial_guard_j;
    map.guard_direction = map.initial_guard_direction.clone();

    loop {
        if visited.contains(&(map.guard_i, map.guard_j, map.guard_direction.clone())) {
            return true;
        } else {
            visited.push((map.guard_i, map.guard_j, map.guard_direction.clone()));
        }
        if map.tick() {
            return false;
        }
    }
}

fn part_2_incomplete_alternative(mut map: Map) -> u32 {
    use Direction::*;
    let mut visited: Vec<(usize, usize, Direction)> = Vec::new();
    let mut found_positions: Vec<(usize, usize)> = Vec::new();

    loop {
        if !visited.contains(&(map.guard_i, map.guard_j, map.guard_direction.clone())) {
            visited.push((map.guard_i, map.guard_j, map.guard_direction.clone()));
        }
        if map.tick() {
            break;
        }
    }

    for (i, j, direction) in visited.clone() {
        if visited
            .iter()
            .filter(|(x, y, _)| *x == i && *y == j)
            .count()
            == 1
        {
            continue;
        }
        if let Some((next_i, next_j, _)) = map.next(i, j, direction.clone()) {
            if next_i == map.initial_guard_i && next_j == map.initial_guard_j {
                continue;
            }
        }
        let turn_direction = direction.turn();
        match turn_direction {
            Up => {
                for row in i..=0 {
                    if visited.contains(&(row, j, turn_direction.clone())) {
                        found_positions.push((row, j));
                        break;
                    }
                }
            }
            Down => {
                for row in i..map.obstacles.len() {
                    if visited.contains(&(row, j, turn_direction.clone())) {
                        found_positions.push((row, j));
                        break;
                    }
                }
            }
            Right => {
                for col in j..map.obstacles[0].len() {
                    if visited.contains(&(i, col, turn_direction.clone())) {
                        found_positions.push((i, col));
                        break;
                    }
                }
            }
            Left => {
                for col in j..=0 {
                    if visited.contains(&(i, col, turn_direction.clone())) {
                        found_positions.push((i, col));
                        break;
                    }
                }
            }
        }
    }
    found_positions.dedup();
    found_positions.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(Map::new(INPUT)), 41);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(Map::new(INPUT)), 6);
    }

    #[test]
    fn test_map() {
        let map = Map::new(INPUT);
        assert_eq!(map.guard_i, 6);
        assert_eq!(map.guard_j, 4);
        assert!(matches!(map.guard_direction, Direction::Up));
    }

    #[test]
    fn test_is_loop() {
        let mut map = Map::new(INPUT);
        map.obstacles[6][3] = true;
        assert!(is_loop(&mut map));
        map.obstacles[6][3] = false;
        map.obstacles[4][4] = true;
        assert!(!is_loop(&mut map));
        map.obstacles[4][4] = false;
        map.obstacles[8][1] = true;
        assert!(is_loop(&mut map));
        map.obstacles[8][1] = false;
        map.obstacles[8][3] = true;
        assert!(is_loop(&mut map));
        map.obstacles[8][3] = false;
        map.obstacles[9][7] = true;
        assert!(is_loop(&mut map));
        map.obstacles[9][7] = false;
    }
}
