use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::Path,
    time::Instant,
    usize,
};

struct Graph {
    data: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Graph {
    fn from_str(input: &str) -> Self {
        let data: Vec<Vec<u8>> = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        let height = data.len();
        let width = data[0].len();
        Self {
            data,
            height,
            width,
        }
    }

    /// Neighbout nodes, ie next to (i, j), within grid bounds and value is one higher
    fn neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        let value = self.data[i][j];
        for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_i = i as i32 + direction.0;
            if new_i < 0 || new_i >= self.height as i32 {
                continue;
            }

            let new_j = j as i32 + direction.1;
            if new_j < 0 || new_j >= self.width as i32 {
                continue;
            }

            if self.data[new_i as usize][new_j as usize] != value + 1 {
                continue;
            }

            neighbours.push((new_i as usize, new_j as usize));
        }
        neighbours
    }
}

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Cannot solve without input!");
    let graph = Graph::from_str(&input);

    let start_1 = Instant::now();
    println!("Part 1: {}, {:?}", part_1(&graph), start_1.elapsed());
    let start_2 = Instant::now();
    println!("Part 2: {}, {:?}", part_2(&graph), start_2.elapsed());
}

fn part_1(graph: &Graph) -> u32 {
    let mut sum = 0;
    for i in 0..graph.data.len() {
        for j in 0..graph.data[i].len() {
            if graph.data[i][j] == 0 {
                sum += trailhead_score(&graph, i, j);
            }
        }
    }
    sum
}

fn part_2(graph: &Graph) -> u32 {
    let mut sum = 0;
    for i in 0..graph.data.len() {
        for j in 0..graph.data[i].len() {
            if graph.data[i][j] == 0 {
                sum += trailhead_score_2(&graph, i, j);
            }
        }
    }
    sum
}

/// BFS, if node == 9, increment score
fn trailhead_score(graph: &Graph, i: usize, j: usize) -> u32 {
    let mut score = 0;
    let mut queue = VecDeque::from([(i, j)]);
    let mut visited = HashSet::from([(i, j)]);
    while !queue.is_empty() {
        let node = queue.pop_front().expect("while !queue.is_empty()");
        if graph.data[node.0][node.1] == 9 {
            score += 1;
        }
        for neighbour in graph.neighbours(node.0, node.1) {
            if visited.get(&neighbour).is_none() {
                visited.insert(neighbour);
                queue.push_back(neighbour);
            }
        }
    }
    score
}

/// BFS without visited, goes through all possible paths to node == 9
fn trailhead_score_2(graph: &Graph, i: usize, j: usize) -> u32 {
    let mut score = 0;
    let mut queue = VecDeque::from([(i, j)]);
    while !queue.is_empty() {
        let node = queue.pop_front().expect("while !queue.is_empty()");
        if graph.data[node.0][node.1] == 9 {
            score += 1;
        }
        for neighbour in graph.neighbours(node.0, node.1) {
            queue.push_back(neighbour);
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let graph = Graph::from_str(
            "5550555
5551555
5552555
6543456
7555557
8555558
9555559",
        );
        assert_eq!(2, part_1(&graph));
        let graph = Graph::from_str(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        assert_eq!(36, part_1(&graph));
    }

    #[test]
    fn test_part_2() {
        let graph = Graph::from_str(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        assert_eq!(81, part_2(&graph));
    }
}
