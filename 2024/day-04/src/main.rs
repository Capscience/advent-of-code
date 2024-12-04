use std::{fs, path::Path};

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Cannot solve without input!");

    let grid = input.lines().map(|line| line.chars().collect()).collect();

    println!("Part 1: {}", part_1(&grid));
    println!("Part 2: {}", part_2(&grid));
}

fn part_1(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'X' {
                count += check_xmas(grid, i, j);
            }
        }
    }
    count
}

fn part_2(grid: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    // Ignore the outermost layer, since A has to be in the middle of the X
    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[0].len() - 1) {
            if grid[i][j] == 'A' {
                if check_x_mas(grid, i, j) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn check_xmas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    let mut count = 0;
    let (min_i, max_i) = (3_usize, grid.len() - 4);
    let (min_j, max_j) = (3_usize, grid[0].len() - 4);

    // Check up and up diagonals
    if i >= min_i {
        // Check up
        if (0..4).map(|k| grid[i - k][j]).collect::<Vec<_>>() == XMAS {
            count += 1;
        }

        if j >= min_j {
            // Check up-left diagonal
            if (0..4).map(|k| grid[i - k][j - k]).collect::<Vec<_>>() == XMAS {
                count += 1;
            }
        }

        if j <= max_j {
            // Check up-right diagonal
            if (0..4).map(|k| grid[i - k][j + k]).collect::<Vec<_>>() == XMAS {
                count += 1;
            }
        }
    }

    // Check down and down diagonals
    if i <= max_i {
        // Check down
        if (0..4).map(|k| grid[i + k][j]).collect::<Vec<_>>() == XMAS {
            count += 1;
        }

        if j >= min_j {
            // Check down-left diagonal
            if (0..4).map(|k| grid[i + k][j - k]).collect::<Vec<_>>() == XMAS {
                count += 1;
            }
        }

        if j <= max_j {
            // Check down-right diagonal
            if (0..4).map(|k| grid[i + k][j + k]).collect::<Vec<_>>() == XMAS {
                count += 1;
            }
        }
    }

    // Check left
    if j >= min_j {
        if (0..4).map(|k| grid[i][j - k]).collect::<Vec<_>>() == XMAS {
            count += 1;
        }
    }

    // Check right
    if j <= max_j {
        if (0..4).map(|k| grid[i][j + k]).collect::<Vec<_>>() == XMAS {
            count += 1;
        }
    }

    count
}

/// i and j must be such, that they are at least 1 away from grid edges
fn check_x_mas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let (top_left, top_right) = (grid[i - 1][j - 1], grid[i - 1][j + 1]);
    let (bottom_left, bottom_right) = (grid[i + 1][j - 1], grid[i + 1][j + 1]);

    // If both diagonals contain both M and S, it is an X-MAS!
    return ((top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M'))
        && ((bottom_left == 'M' && top_right == 'S') || (bottom_left == 'S' && top_right == 'M'));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_check_xmas() {
        let grid: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(check_xmas(&grid, 0, 0), 0);
        assert_eq!(check_xmas(&grid, 0, 4), 1);
        assert_eq!(check_xmas(&grid, 4, 6), 2);
    }

    #[test]
    fn test_part_1() {
        let grid: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(part_1(&grid), 18);
    }

    #[test]
    fn test_part_2() {
        let grid: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(part_2(&grid), 9);
    }
}
