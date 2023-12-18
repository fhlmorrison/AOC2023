use std::collections::{BinaryHeap, HashSet};
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    // let reader = read_to_string("./inputs/test17A.txt").unwrap();
    let reader = read_to_string("./inputs/day17.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let start = Instant::now();
    let matrix = reader
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum = djikstra(&matrix);

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

fn part_2(reader: &str) {
    let start = Instant::now();
    let matrix = reader
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum = djikstra2(&matrix);

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

#[derive(Clone, Copy, Debug, Hash)]
struct Node {
    x: usize,
    y: usize,
    g: usize,
    vx: isize,
    vy: isize,
    n: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.g.cmp(&self.g)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.g.cmp(&self.g))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.g == other.g
    }
}
impl Eq for Node {}

fn djikstra(matrix: &Vec<Vec<usize>>) -> usize {
    let mut open_nodes = BinaryHeap::new();

    let mut visited = HashSet::new();

    let start_node = Node {
        g: 0,
        x: 0,
        y: 0,
        vx: 0,
        vy: 0,
        n: 0,
    };

    open_nodes.push(start_node);

    while let Some(current) = open_nodes.pop() {
        let Node { g, x, y, vx, vy, n } = current;

        // Ending condition
        if current.y == matrix.len() - 1 && current.x == matrix[0].len() - 1 {
            return g;
        }

        if visited.contains(&(x, y, vx, vy, n)) {
            continue;
        }
        visited.insert((x, y, vx, vy, n));

        // Add continuing case
        if n < 3 && (vx, vy) != (0, 0) {
            let nx = x as isize + vx;
            let ny = y as isize + vy;
            if ny >= 0 && nx >= 0 && ny < matrix.len() as isize && nx < matrix[0].len() as isize {
                let new_node = Node {
                    g: g + matrix[ny as usize][nx as usize],
                    x: nx as usize,
                    y: ny as usize,
                    vx: vx,
                    vy: vy,
                    n: n + 1,
                };
                open_nodes.push(new_node);
            }
        }

        // Add turning cases
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            // println!("dx: {}, dy: {}", dx, dy);
            if dx == vx && dy == vy {
                // Not turning
                continue;
            }
            if dx == -vx && dy == -vy {
                // 180 turn
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0 || ny < 0 || nx >= matrix[0].len() as isize || ny >= matrix.len() as isize {
                continue;
            }

            let new_node = Node {
                g: g + matrix[ny as usize][nx as usize],
                x: nx as usize,
                y: ny as usize,
                vx: dx,
                vy: dy,
                n: 1,
            };
            open_nodes.push(new_node);
        }
    }

    usize::MAX
}

fn djikstra2(matrix: &Vec<Vec<usize>>) -> usize {
    let mut open_nodes = BinaryHeap::new();

    let mut visited = HashSet::new();

    let start_node = Node {
        g: 0,
        x: 0,
        y: 0,
        vx: 0,
        vy: 0,
        n: 0,
    };

    open_nodes.push(start_node);

    while let Some(current) = open_nodes.pop() {
        // visualize_path(&visited, matrix, &current);

        let Node { g, x, y, vx, vy, n } = current;

        // Ending condition
        if current.y == matrix.len() - 1 && current.x == matrix[0].len() - 1 && n >= 4 {
            return g;
        }

        if visited.contains(&(x, y, vx, vy, n)) {
            continue;
        }
        visited.insert((x, y, vx, vy, n));

        // Add continuing case
        if n < 10 && (vx, vy) != (0, 0) {
            let nx = x as isize + vx;
            let ny = y as isize + vy;
            if ny >= 0 && nx >= 0 && ny < matrix.len() as isize && nx < matrix[0].len() as isize {
                let new_node = Node {
                    g: g + matrix[ny as usize][nx as usize],
                    x: nx as usize,
                    y: ny as usize,
                    vx: vx,
                    vy: vy,
                    n: n + 1,
                };
                open_nodes.push(new_node);
            }
        }

        if n >= 4 || (vx, vy) == (0, 0) {
            // Add turning cases
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                // println!("dx: {}, dy: {}", dx, dy);
                if dx == vx && dy == vy {
                    // Not turning
                    continue;
                }
                if dx == -vx && dy == -vy {
                    // 180 turn
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || ny < 0 || nx >= matrix[0].len() as isize || ny >= matrix.len() as isize
                {
                    continue;
                }

                let new_node = Node {
                    g: g + matrix[ny as usize][nx as usize],
                    x: nx as usize,
                    y: ny as usize,
                    vx: dx,
                    vy: dy,
                    n: 1,
                };
                open_nodes.push(new_node);
            }
        }
    }

    usize::MAX
}
