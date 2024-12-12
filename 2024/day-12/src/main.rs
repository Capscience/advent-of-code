use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::Path,
    time::Instant,
};

static DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

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
            .map(|line| line.bytes().collect())
            .collect();
        let height = data.len();
        let width = data[0].len();
        Self {
            data,
            height,
            width,
        }
    }

    /// Neighbout nodes, ie next to (i, j) and within grid bounds, and same value
    fn neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        let value = self.data[i][j];
        for direction in DIRECTIONS {
            let new_i = i as i32 + direction.0;
            if new_i < 0 || new_i >= self.height as i32 {
                continue;
            }

            let new_j = j as i32 + direction.1;
            if new_j < 0 || new_j >= self.width as i32 {
                continue;
            }

            if self.data[new_i as usize][new_j as usize] != value {
                continue;
            }

            neighbours.push((new_i as usize, new_j as usize));
        }
        neighbours
    }

    /// Count fences which visited neighbours have not counted yet
    fn fence_cost(&self, i: usize, j: usize, visited: &mut HashSet<(usize, usize)>) -> u32 {
        let mut fences = self.fences(i, j);
        for neighbour in self.neighbours(i, j) {
            if visited.get(&neighbour).is_some() {
                fences.retain(|fence| self.fences(neighbour.0, neighbour.1).get(fence).is_none());
            }
        }
        fences.len() as u32
    }

    /// Get directions where the fences are, ie. where there are no neighbours
    fn fences(&self, i: usize, j: usize) -> HashSet<(i32, i32)> {
        let mut fences = HashSet::from(DIRECTIONS);
        for neighbour in self.neighbours(i, j) {
            let neighbour_direction =
                (neighbour.0 as i32 - i as i32, neighbour.1 as i32 - j as i32);
            fences.retain(|dir| *dir != neighbour_direction);
        }
        fences
    }

    /// BFS with area and perimeter calculation
    fn plot_cost(&self, i: usize, j: usize, visited: &mut HashSet<(usize, usize)>) -> u32 {
        let mut area = 0;
        let mut perimeter = 0;
        visited.insert((i, j));
        let mut queue = VecDeque::from([(i, j)]);
        while !queue.is_empty() {
            let node = queue.pop_front().expect("while !queue.is_empty()");
            let neighbours = self.neighbours(node.0, node.1);
            area += 1;
            perimeter += 4 - neighbours.len() as u32;
            for neighbour in neighbours {
                if visited.get(&neighbour).is_none() {
                    visited.insert(neighbour);
                    queue.push_back(neighbour);
                }
            }
        }
        area * perimeter
    }

    /// BFS with area and custom perimeter calculation
    /// `visited.insert(...)` moved to prevent messing up `self.fence_cost` result
    fn plot_cost_2(&self, i: usize, j: usize, visited: &mut HashSet<(usize, usize)>) -> u32 {
        let mut area = 0;
        let mut perimeter = 0;
        let mut queue = VecDeque::from([(i, j)]);
        while !queue.is_empty() {
            let node = queue.pop_front().expect("while !queue.is_empty()");
            if visited.get(&node).is_some() {
                continue;
            }
            visited.insert(node);
            let neighbours = self.neighbours(node.0, node.1);
            area += 1;
            perimeter += self.fence_cost(node.0, node.1, visited);
            for neighbour in neighbours {
                if visited.get(&neighbour).is_none() {
                    queue.push_back(neighbour);
                }
            }
        }
        area * perimeter
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
    let mut cost = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..graph.height {
        for j in 0..graph.width {
            if visited.get(&(i, j)).is_none() {
                cost += graph.plot_cost(i, j, &mut visited);
            }
        }
    }
    cost
}

fn part_2(graph: &Graph) -> u32 {
    let mut cost = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..graph.height {
        for j in 0..graph.width {
            if visited.get(&(i, j)).is_none() {
                cost += graph.plot_cost_2(i, j, &mut visited);
            }
        }
    }
    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    const G1: &str = "AAAA
BBCD
BBCC
EEEC";

    const G2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const G3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const G4: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_part_1() {
        let graph = Graph::from_str(G1);
        assert_eq!(part_1(&graph), 140);
        let graph = Graph::from_str(G2);
        assert_eq!(part_1(&graph), 772);
        let graph = Graph::from_str(G3);
        assert_eq!(part_1(&graph), 1930);
    }

    #[test]
    fn test_plot_cost_2() {
        let graph = Graph::from_str(G1);
        let mut visited = HashSet::new();
        assert_eq!(graph.plot_cost_2(0, 0, &mut visited), 16);
        assert_eq!(graph.plot_cost_2(1, 0, &mut visited), 16);
        assert_eq!(graph.plot_cost_2(3, 0, &mut visited), 12);
        assert_eq!(graph.plot_cost_2(1, 2, &mut visited), 32);
        assert_eq!(graph.plot_cost_2(1, 3, &mut visited), 4);
    }

    #[test]
    fn test_plot_fences() {
        let graph = Graph::from_str(G1);
        assert_eq!(
            graph.fences(0, 0),
            HashSet::from([(-1, 0), (0, -1), (1, 0)])
        );
        assert_eq!(graph.fences(0, 1), HashSet::from([(-1, 0), (1, 0)]));
        assert_eq!(graph.fences(0, 2), HashSet::from([(-1, 0), (1, 0)]));
        assert_eq!(graph.fences(0, 3), HashSet::from([(-1, 0), (0, 1), (1, 0)]));
    }

    #[test]
    fn test_fence_cost() {
        let graph = Graph::from_str(G1);
        let mut visited = HashSet::new();
        assert_eq!(graph.fence_cost(0, 0, &mut visited), 3);
        visited.insert((0, 0));
        assert_eq!(graph.fence_cost(0, 1, &mut visited), 0);
        visited.insert((0, 1));
        assert_eq!(graph.fence_cost(0, 2, &mut visited), 0);
        visited.insert((0, 2));
        assert_eq!(graph.fence_cost(0, 3, &mut visited), 1);
    }

    #[test]
    fn test_part_2() {
        let graph = Graph::from_str(G1);
        assert_eq!(part_2(&graph), 80);
        let graph = Graph::from_str(G2);
        assert_eq!(part_2(&graph), 436);
        let graph = Graph::from_str(G3);
        assert_eq!(part_2(&graph), 1206);
        let graph = Graph::from_str(G4);
        assert_eq!(part_2(&graph), 368);
    }
}
