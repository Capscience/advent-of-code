use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    path::Path,
    time::Instant,
};

use glam::IVec2;

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Cannot solve without input!");

    let start_1 = Instant::now();
    println!("Part 1: {}, {:?}", part_1(&input), start_1.elapsed());
    let start_2 = Instant::now();
    println!("Part 2: {}, {:?}", part_2(&input), start_2.elapsed());
}

fn part_1(input: &str) -> i32 {
    let (mut robot, moves) = parse(input, false);
    for step in moves.chars() {
        robot.try_move(step);
    }
    robot
        .boxes
        .iter()
        .map(|(position, _)| 100 * position.y + position.x)
        .sum()
}

fn part_2(input: &str) -> i32 {
    let (mut robot, moves) = parse(input, true);

    for step in moves.chars() {
        robot.try_move(step);
    }
    robot.coordinate_sum()
}

fn parse(input: &str, double_width: bool) -> (Robot, &str) {
    let mut parts = input.split("\n\n");
    (
        Robot::new(parts.next().unwrap(), double_width),
        parts.next().unwrap(),
    )
}

struct Robot {
    position: IVec2,
    walls: HashMap<IVec2, IVec2>,
    boxes: HashMap<IVec2, IVec2>,
    double_width: bool,
}

impl Robot {
    fn new(grid: &str, double_width: bool) -> Self {
        let mut position = IVec2::ZERO;
        let mut walls = HashMap::new();
        let mut boxes = HashMap::new();
        for (y, row) in grid.lines().enumerate() {
            for (x, symbol) in row.chars().enumerate() {
                match symbol {
                    '#' => {
                        if double_width {
                            let _ = walls.insert(
                                IVec2::new((x as i32) * 2, y as i32),
                                IVec2::new((x as i32) * 2 + 1, y as i32),
                            );
                            let _ = walls.insert(
                                IVec2::new((x as i32) * 2 + 1, y as i32),
                                IVec2::new((x as i32) * 2, y as i32),
                            );
                        } else {
                            let _ = walls.insert(
                                IVec2::new(x as i32, y as i32),
                                IVec2::new(x as i32, y as i32),
                            );
                        }
                    }
                    '@' => {
                        if double_width {
                            position = IVec2::new((x as i32) * 2, y as i32);
                        } else {
                            position = IVec2::new(x as i32, y as i32);
                        }
                    }
                    'O' => {
                        if double_width {
                            let _ = boxes.insert(
                                IVec2::new((x as i32) * 2, y as i32),
                                IVec2::new((x as i32) * 2 + 1, y as i32),
                            );
                            let _ = boxes.insert(
                                IVec2::new((x as i32) * 2 + 1, y as i32),
                                IVec2::new((x as i32) * 2, y as i32),
                            );
                        } else {
                            let _ = boxes.insert(
                                IVec2::new(x as i32, y as i32),
                                IVec2::new(x as i32, y as i32),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
        Self {
            position,
            walls,
            boxes,
            double_width,
        }
    }

    /// Try to move, no-op if not possible
    fn try_move(&mut self, direction: char) {
        let dir_vec = match direction {
            '^' => IVec2::NEG_Y,
            '>' => IVec2::X,
            'v' => IVec2::Y,
            '<' => IVec2::NEG_X,
            _ => return,
        };
        let new_position = self.position + dir_vec;
        if self.walls.contains_key(&new_position) {
            return;
        } else if !self.boxes.contains_key(&new_position) {
            self.position = new_position;
            return;
        }
        if self.double_width {
            self.try_move_2(dir_vec);
        } else {
            self.try_move_1(dir_vec);
        }
    }

    fn try_move_1(&mut self, dir_vec: IVec2) {
        let new_position = self.position + dir_vec;

        let mut check_position = new_position + dir_vec;
        while self.boxes.contains_key(&check_position) {
            check_position += dir_vec;
        }

        if !self.walls.contains_key(&check_position) {
            self.position = new_position;
            self.boxes.remove(&new_position);
            self.boxes.insert(check_position, check_position);
        }
    }

    fn try_move_2(&mut self, dir_vec: IVec2) {
        let new_position = self.position + dir_vec;
        if !self.boxes.contains_key(&new_position) {
            return;
        }
        let mut queue: VecDeque<IVec2> = VecDeque::from([new_position]);
        let mut to_move = HashSet::new();
        let mut movable = true;

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let neighbour = *self.boxes.get(&current).unwrap();

            if self.walls.get(&(current + dir_vec)).is_some()
                || self.walls.get(&(neighbour + dir_vec)).is_some()
            {
                movable = false;
                break;
            }

            to_move.insert(sorted(current, neighbour));

            if let Some(&current_push) = self.boxes.get(&(current + dir_vec)) {
                let other = self.boxes.get(&current_push).unwrap();
                let (first, second) = sorted(current_push, *other);
                if !to_move.contains(&(first, second)) {
                    to_move.insert(sorted(current_push, *other));
                    queue.push_back(current_push);
                }
            };
            if let Some(&current_push) = self.boxes.get(&(neighbour + dir_vec)) {
                let other = self.boxes.get(&current_push).unwrap();
                let (first, second) = sorted(current_push, *other);
                if !to_move.contains(&(first, second)) {
                    to_move.insert(sorted(current_push, *other));
                    queue.push_back(current_push);
                }
            };
        }

        if !movable {
            return;
        }

        // Movement possible, execute!
        self.position = new_position;
        for &(left, right) in to_move.iter() {
            self.boxes.remove(&left);
            self.boxes.remove(&right);
        }
        for &(left, right) in to_move.iter() {
            let new_left = left + dir_vec;
            let new_right = right + dir_vec;
            self.boxes.insert(new_left, new_right);
            self.boxes.insert(new_right, new_left);
        }
    }

    fn coordinate_sum(&self) -> i32 {
        let mut coordinates = HashSet::new();
        for (&key, &value) in &self.boxes {
            let (left, _) = sorted(key, value);
            coordinates.insert(left);
        }
        coordinates
            .iter()
            .map(|position| 100 * position.y + position.x)
            .sum()
    }
}

fn sorted(a: IVec2, b: IVec2) -> (IVec2, IVec2) {
    if a.x < b.x || a.y < b.y {
        (a, b)
    } else {
        (b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const INPUT2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const INPUT3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT1), 2028);
        assert_eq!(part_1(INPUT2), 10092);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT3), 618);
        assert_eq!(part_2(INPUT2), 9021);
    }

    #[test]
    fn test_to_move_2() {
        let mut robot = Robot::new(INPUT3, true);
        robot.try_move('<');
        assert_eq!(
            robot.boxes,
            HashMap::from([
                (IVec2::new(5, 3), IVec2::new(6, 3)),
                (IVec2::new(6, 3), IVec2::new(5, 3)),
                (IVec2::new(7, 3), IVec2::new(8, 3)),
                (IVec2::new(8, 3), IVec2::new(7, 3)),
                (IVec2::new(6, 4), IVec2::new(7, 4)),
                (IVec2::new(7, 4), IVec2::new(6, 4)),
            ])
        );
        robot.try_move('v');
        robot.try_move('v');
        robot.try_move('<');
        robot.try_move('<');
        assert_eq!(
            robot.boxes,
            HashMap::from([
                (IVec2::new(5, 3), IVec2::new(6, 3)),
                (IVec2::new(6, 3), IVec2::new(5, 3)),
                (IVec2::new(7, 3), IVec2::new(8, 3)),
                (IVec2::new(8, 3), IVec2::new(7, 3)),
                (IVec2::new(6, 4), IVec2::new(7, 4)),
                (IVec2::new(7, 4), IVec2::new(6, 4)),
            ])
        );
        robot.try_move('^');
        assert_eq!(
            robot.boxes,
            HashMap::from([
                (IVec2::new(5, 2), IVec2::new(6, 2)),
                (IVec2::new(6, 2), IVec2::new(5, 2)),
                (IVec2::new(7, 2), IVec2::new(8, 2)),
                (IVec2::new(8, 2), IVec2::new(7, 2)),
                (IVec2::new(6, 3), IVec2::new(7, 3)),
                (IVec2::new(7, 3), IVec2::new(6, 3)),
            ])
        );
        robot.try_move('^');
        assert_eq!(
            robot.boxes,
            HashMap::from([
                (IVec2::new(5, 2), IVec2::new(6, 2)),
                (IVec2::new(6, 2), IVec2::new(5, 2)),
                (IVec2::new(7, 2), IVec2::new(8, 2)),
                (IVec2::new(8, 2), IVec2::new(7, 2)),
                (IVec2::new(6, 3), IVec2::new(7, 3)),
                (IVec2::new(7, 3), IVec2::new(6, 3)),
            ])
        );
        robot.try_move('<');
        robot.try_move('<');
        robot.try_move('^');
        robot.try_move('^');
        assert_eq!(
            robot.boxes,
            HashMap::from([
                (IVec2::new(5, 1), IVec2::new(6, 1)),
                (IVec2::new(6, 1), IVec2::new(5, 1)),
                (IVec2::new(7, 2), IVec2::new(8, 2)),
                (IVec2::new(8, 2), IVec2::new(7, 2)),
                (IVec2::new(6, 3), IVec2::new(7, 3)),
                (IVec2::new(7, 3), IVec2::new(6, 3)),
            ])
        );
    }
}
