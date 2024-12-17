use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
    path::Path,
    time::Instant,
};

use glam::IVec2;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

fn main() {
    let path = Path::new("input.txt");
    let input = parse(&fs::read_to_string(path).expect("Cannot solve without input!"));

    let start_1 = Instant::now();
    println!("Part 1: {}, {:?}", part_1(&input), start_1.elapsed());
    let start_2 = Instant::now();
    println!("Part 2: {}, {:?}", part_2(&input), start_2.elapsed());
}

/// Dijkstra
fn part_1(graph: &Graph) -> i32 {
    let mut distances = graph.init_distances();
    let mut queue = BinaryHeap::new();
    queue.push(HeapEntry::new(0, graph.start));

    while !queue.is_empty() {
        let current = queue.pop().expect("while !queue.is_empty()");
        for (new, weight) in graph.neighbours(current.node) {
            let node_distance = distances
                .get(&new)
                .expect("all nodes should have a distance");
            let new_distance = distances
                .get(&current.node)
                .expect("all nodes should have a distance")
                + weight;
            if new_distance < *node_distance {
                distances.insert(new, new_distance);
                queue.push(HeapEntry::new(new_distance, new));
            }
        }
    }

    *distances
        .iter()
        .filter(|(node, _)| node.pos == graph.end)
        .map(|(_, distance)| distance)
        .min()
        .expect("Should have an answer")
}

/// Dijkstra with tracing of all best paths
fn part_2(graph: &Graph) -> i32 {
    let mut distances = graph.init_distances();
    let mut previous = graph.init_previous();
    let mut queue = BinaryHeap::new();
    queue.push(HeapEntry::new(0, graph.start));

    while !queue.is_empty() {
        let current = queue.pop().expect("while !queue.is_empty()");
        for (new, weight) in graph.neighbours(current.node) {
            let node_distance = distances
                .get(&new)
                .expect("all nodes should have a distance");
            let new_distance = distances
                .get(&current.node)
                .expect("all nodes should have a distance")
                + weight;

            use std::cmp::Ordering::*;
            match new_distance.cmp(node_distance) {
                Less => {
                    previous.insert(new, Some(vec![current.node]));
                    distances.insert(new, new_distance);
                    queue.push(HeapEntry::new(new_distance, new));
                }
                Equal => {
                    if let Some(Some(nodes)) = previous.get_mut(&new) {
                        nodes.push(current.node);
                    }
                }
                Greater => {}
            }
        }
    }
    let path = get_path(&previous, &distances, graph.start, graph.end);
    path.len() as i32
}

fn get_path(
    previous: &HashMap<Node, Option<Vec<Node>>>,
    distances: &HashMap<Node, i32>,
    start: Node,
    end_pos: IVec2,
) -> HashSet<IVec2> {
    let mut min = i32::MAX;
    let mut all_goals = Vec::new();
    for node in DIRECTIONS.map(|dir| Node::new(end_pos, dir)) {
        use std::cmp::Ordering::*;
        let dist = distances.get(&node).unwrap();
        match dist.cmp(&min) {
            Less => {
                min = *dist;
                all_goals = vec![node];
            }
            Equal => all_goals.push(node),
            Greater => {}
        }
    }
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from(all_goals);
    while !queue.is_empty() {
        let node = queue.pop_front().expect("while !queue.is_empty()");
        visited.insert(node);
        if let Some(Some(prev)) = previous.get(&node) {
            for option in prev {
                if !visited.contains(option) {
                    queue.push_back(*option);
                }
            }
        } else if node != start {
            panic!("Path didn't start from the Start node!");
        }
    }
    visited.iter().map(|node| node.pos).collect()
}

fn parse(input: &str) -> Graph {
    let mut nodes = HashSet::new();
    let mut start = Node::new(IVec2::ZERO, IVec2::X);
    let mut end = IVec2::ZERO;

    for (i, row) in input.lines().enumerate() {
        for (j, c) in row.chars().enumerate() {
            match c {
                '.' => {
                    for dir in DIRECTIONS {
                        let _ = nodes.insert(Node::new(IVec2::new(j as i32, i as i32), dir));
                    }
                }
                'S' => {
                    start = Node::new(IVec2::new(j as i32, i as i32), IVec2::X);
                    for dir in DIRECTIONS {
                        let _ = nodes.insert(Node::new(IVec2::new(j as i32, i as i32), dir));
                    }
                }
                'E' => {
                    end = IVec2::new(j as i32, i as i32);
                    for dir in DIRECTIONS {
                        let _ = nodes.insert(Node::new(IVec2::new(j as i32, i as i32), dir));
                    }
                }
                '#' => {}
                other => panic!("Unexpected character {other}!"),
            }
        }
    }

    Graph { nodes, start, end }
}

/// Needed for proper sorting of BinaryHeap
#[derive(PartialEq, Eq, Debug)]
struct HeapEntry {
    distance: i32,
    node: Node,
}

impl HeapEntry {
    fn new(distance: i32, node: Node) -> Self {
        Self { distance, node }
    }
}

/// Reverse ordering to get a min-heap
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Reverse ordering to get a min-heap
impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.distance).cmp(&Reverse(other.distance))
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Node {
    pos: IVec2,
    dir: IVec2,
}

impl Node {
    fn new(pos: IVec2, dir: IVec2) -> Self {
        Self { pos, dir }
    }

    fn next(&self) -> Self {
        Self {
            pos: self.pos + self.dir,
            dir: self.dir,
        }
    }
}

struct Graph {
    nodes: HashSet<Node>,
    start: Node,
    end: IVec2,
}

impl Graph {
    fn neighbours(&self, node: Node) -> Vec<(Node, i32)> {
        let mut neighbours = Vec::new();
        if self.nodes.contains(&node.next()) {
            neighbours.push((node.next(), 1));
        }
        // Add 90 degree rotations, continue if 0 or 180 degrees
        for new_dir in DIRECTIONS {
            if new_dir == node.dir || new_dir == -node.dir {
                continue;
            }
            neighbours.push((Node::new(node.pos, new_dir), 1000));
        }
        neighbours
    }

    fn init_distances(&self) -> HashMap<Node, i32> {
        self.nodes
            .iter()
            .map(|&node| {
                if node == self.start {
                    (node, 0)
                } else {
                    (node, i32::MAX)
                }
            })
            .collect()
    }

    fn init_previous(&self) -> HashMap<Node, Option<Vec<Node>>> {
        self.nodes.iter().map(|&node| (node, None)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_ord() {
        let mut q = BinaryHeap::new();
        q.push(HeapEntry::new(
            1000,
            Node::new(IVec2::new(1, 13), IVec2::NEG_Y),
        ));
        q.push(HeapEntry::new(1, Node::new(IVec2::new(2, 13), IVec2::X)));
        q.push(HeapEntry::new(1000, Node::new(IVec2::new(1, 13), IVec2::Y)));
        assert_eq!(
            q.pop(),
            Some(HeapEntry::new(1, Node::new(IVec2::new(2, 13), IVec2::X)))
        );
        q.push(HeapEntry::new(2, Node::new(IVec2::new(3, 13), IVec2::X)));
        assert_eq!(
            q.pop(),
            Some(HeapEntry::new(2, Node::new(IVec2::new(3, 13), IVec2::X)))
        );
        assert_eq!(q.pop().unwrap().distance, 1000);
    }

    #[test]
    fn test_part_1() {
        let graph = parse(INPUT1);
        assert_eq!(part_1(&graph), 7036);
        let graph = parse(INPUT2);
        assert_eq!(part_1(&graph), 11048);
    }

    #[test]
    fn test_part_2() {
        let graph = parse(INPUT1);
        assert_eq!(part_2(&graph), 45);
        let graph = parse(INPUT2);
        assert_eq!(part_2(&graph), 64);
    }
}
