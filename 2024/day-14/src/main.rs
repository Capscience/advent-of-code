use std::{
    collections::{HashSet, VecDeque},
    fs,
    io::{self, stdin, BufRead, Write},
    path::Path,
    time::Instant,
};

use terminal_size::{terminal_size, Height};

const CANDIDATE_MINIMUM_SIZE: u32 = 15;

fn main() {
    let path = Path::new("input.txt");
    let input: Vec<Robot> = fs::read_to_string(path)
        .expect("Cannot solve without input!")
        .lines()
        .map(|machine| Robot::new(machine, 101, 103))
        .collect();

    let start_1 = Instant::now();
    let part_1_result = format!(
        "Part 1: {}, {:?}\n",
        part_1(&input, 101, 103),
        start_1.elapsed()
    );
    part_2(&input, &part_1_result);
}

fn part_1(input: &[Robot], width: i32, height: i32) -> i32 {
    let positions = input
        .iter()
        .map(|robot| robot.position(100))
        .collect::<HashSet<(i32, i32)>>();
    let mut quadrants = [0, 0, 0, 0];
    for position in positions {
        if let Some(index) = quadrant(position.0, position.1, width, height) {
            quadrants[index] += 1;
        }
    }
    quadrants.iter().product()
}

/// Render candidates to terminal, Enter gives you the next candidate, Ctrl-C stops the program
fn part_2(input: &[Robot], part_1_result: &str) {
    let mut grid = [['.'; 101]; 103];
    let mut second = 0;
    let mut stdout = io::stdout().lock();
    let mut start = Instant::now();

    // Clear terminal and move to top left
    stdout.write_all(b"\x1B[2J\x1B[H").unwrap();
    // Write part 1 result
    stdout.write_all(part_1_result.as_bytes()).unwrap();
    // Write instructions
    stdout
        .write_all("Press Enter for next candidate, Ctrl-C to exit.\n".as_bytes())
        .unwrap();

    loop {
        let positions = input
            .iter()
            .map(|robot| robot.position(second))
            .collect::<HashSet<(i32, i32)>>();

        // Only render potential results
        if largest_robot_cluster(&positions) < CANDIDATE_MINIMUM_SIZE {
            second += 1;
            continue;
        }
        // Clear terminal and move to top left
        stdout.write_all(b"\x1B[2J\x1B[H").unwrap();

        // Set robot positions in grid
        for (x, y) in &positions {
            grid[*y as usize][*x as usize] = 'O';
        }

        for line in grid.iter().map(|chars| {
            let mut row = chars.iter().collect::<String>();
            row.push('\n');
            row
        }) {
            let _ = stdout.write_all(line.as_bytes());
        }

        // Keep part 1 result on the screen
        stdout.write_all(part_1_result.as_bytes()).unwrap();
        // Write instructions
        stdout
            .write_all("Press Enter for next candidate, Ctrl-C to exit.\n".as_bytes())
            .unwrap();
        stdout
            .write_all(
                format!(
                    "Terminal height: {}, 106 lines rendered, second {}, {:?}\n",
                    terminal_height(),
                    second,
                    start.elapsed(),
                )
                .as_bytes(),
            )
            .unwrap();

        // Reset grid
        for (x, y) in &positions {
            grid[*y as usize][*x as usize] = '.';
        }
        second += 1;
        let _ = stdin().lock().read_line(&mut "".to_string());
        start = Instant::now();
    }
}

fn largest_robot_cluster(positions: &HashSet<(i32, i32)>) -> u32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut highest: u32 = 0;
    for position in positions {
        if visited.get(&position).is_some() {
            continue;
        }
        let size = graph_size(*position, &positions, &mut visited);
        if size > CANDIDATE_MINIMUM_SIZE {
            return size;
        }
        if size > highest {
            highest = size;
        }
    }
    highest
}

fn graph_size(
    position: (i32, i32),
    nodes: &HashSet<(i32, i32)>,
    visited: &mut HashSet<(i32, i32)>,
) -> u32 {
    const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut size = 0;
    let mut queue = VecDeque::from([position]);
    visited.insert(position);
    while !queue.is_empty() {
        let node = queue.pop_front().expect("while !queue.is_empty()");
        size += 1;
        if size > CANDIDATE_MINIMUM_SIZE {
            return size;
        }
        for dir in DIRECTIONS {
            let new_pos = (node.0 + dir.0, node.1 + dir.1);
            if nodes.get(&new_pos).is_some() && visited.get(&new_pos).is_none() {
                queue.push_back(new_pos);
                visited.insert(new_pos);
            }
        }
    }
    size
}

fn terminal_height() -> u16 {
    terminal_size().map(|(_, Height(h))| h).unwrap()
}

/// Get quadrant index for a position in <width>x<height> space.
/// TL = 0, TR = 1, BL = 2, BR = 3
/// If position is exactly on the middle line, return None.
fn quadrant(x: i32, y: i32, width: i32, height: i32) -> Option<usize> {
    let middle_w = width / 2;
    let middle_h = height / 2;
    if x == middle_w || y == middle_h {
        None
    } else {
        let quadrant = y / (middle_h + 1) * 2 + x / (middle_w + 1);
        Some(quadrant as usize)
    }
}

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
    width: i32,
    height: i32,
}

impl Robot {
    fn new(input: &str, width: i32, height: i32) -> Self {
        let parsed: Vec<Vec<i32>> = input
            .split_whitespace()
            .map(|part| {
                part[2..]
                    .split(',')
                    .map(|number| number.parse().unwrap())
                    .collect()
            })
            .collect();
        Self {
            x: parsed[0][0],
            y: parsed[0][1],
            vx: parsed[1][0],
            vy: parsed[1][1],
            width,
            height,
        }
    }

    fn position(&self, seconds: i32) -> (i32, i32) {
        let x = (self.x + seconds * self.vx)
            .checked_rem_euclid(self.width)
            .unwrap();
        let y = (self.y + seconds * self.vy)
            .checked_rem_euclid(self.height)
            .unwrap();
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot() {
        let robot = Robot::new("p=2,4 v=2,-3", 11, 7);
        assert_eq!(robot.position(5), (1, 3));
    }

    #[test]
    fn test_part_1() {
        let input: Vec<Robot> = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
            .lines()
            .map(|machine| Robot::new(machine, 11, 7))
            .collect();
        assert_eq!(part_1(&input, 11, 7), 12);
    }
}
