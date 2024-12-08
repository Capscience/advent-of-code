use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
    time::Instant,
};

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Cannot solve without input!");

    let antennas = antennas(&input);

    let start_1 = Instant::now();
    println!(
        "Part 1: {}, {:?}",
        part_1(
            &antennas,
            input.lines().count(),
            input.lines().next().unwrap().chars().count()
        ),
        start_1.elapsed()
    );
    let start_2 = Instant::now();
    println!(
        "Part 2: {}, {:?}",
        part_2(
            &antennas,
            input.lines().count(),
            input.lines().next().unwrap().chars().count()
        ),
        start_2.elapsed()
    );
}

fn part_1(antennas: &HashMap<char, Vec<(usize, usize)>>, height: usize, width: usize) -> usize {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    antennas.values().for_each(|locations| {
        get_antinodes(locations, &mut antinodes, height, width);
    });
    antinodes.len()
}

fn part_2(antennas: &HashMap<char, Vec<(usize, usize)>>, height: usize, width: usize) -> usize {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    antennas.values().for_each(|locations| {
        get_antinodes_2(locations, &mut antinodes, height, width);
    });
    antinodes.len()
}

fn get_antinodes(
    locations: &[(usize, usize)],
    antinodes: &mut HashSet<(usize, usize)>,
    height: usize,
    width: usize,
) {
    for location in locations {
        for other in locations {
            if location == other {
                continue;
            }
            if let Some(mirror) = get_mirror(*location, *other, height, width) {
                antinodes.insert(mirror);
            }
        }
    }
}

fn get_antinodes_2(
    locations: &[(usize, usize)],
    antinodes: &mut HashSet<(usize, usize)>,
    height: usize,
    width: usize,
) {
    for location in locations {
        for other in locations {
            if location == other {
                continue;
            }
            let mut current = *location;
            let mut point_to_mirror = *other;
            antinodes.insert(current); // Make sure that the location of the antenna is included
            antinodes.insert(point_to_mirror); // Same for the other antenna

            while let Some(mirror) = get_mirror(current, point_to_mirror, height, width) {
                antinodes.insert(mirror);
                point_to_mirror = current;
                current = mirror;
            }
        }
    }
}

fn get_mirror(
    location: (usize, usize),
    other: (usize, usize),
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    let diff = (
        location.0 as i32 - other.0 as i32,
        location.1 as i32 - other.1 as i32,
    );
    let mirror = (location.0 as i32 + diff.0, location.1 as i32 + diff.1);
    if mirror.0 < 0 || mirror.0 >= height as i32 || mirror.1 < 0 || mirror.1 >= width as i32 {
        return None;
    }
    Some((mirror.0 as usize, mirror.1 as usize))
}

fn antennas(input: &str) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in input.lines().enumerate() {
        for (j, char) in row.chars().enumerate() {
            match char {
                '.' | '#' => continue,
                _ => {
                    if let Some(positions) = antennas.get_mut(&char) {
                        positions.push((i, j));
                    } else {
                        antennas.insert(char, vec![(i, j)]);
                    }
                }
            }
        }
    }
    antennas
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_1() {
        let antennas = antennas(INPUT);
        assert_eq!(part_1(&antennas, 12, 12), 14);
    }

    #[test]
    fn test_part_2() {
        let antennas = antennas(INPUT);
        assert_eq!(part_2(&antennas, 12, 12), 34);
    }

    #[test]
    fn test_part_2_t() {
        let antennas = antennas(
            "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........",
        );
        assert_eq!(part_2(&antennas, 10, 10), 9);
    }
}
