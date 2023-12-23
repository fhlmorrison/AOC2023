use fxhash::{FxHashMap, FxHashSet}; // For faster hashing
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    // let reader = read_to_string("./inputs/test23A.txt").unwrap();
    let reader = read_to_string("./inputs/day23.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}
fn part_1(reader: &str) {
    let start = Instant::now();
    let lines = reader.lines();

    let matrix = lines
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_tile = (1, 0); // (x, y)
    let goal = (matrix[0].len() - 2, matrix.len() - 1);

    let mut queue = VecDeque::<Path>::new();
    // let mut longest: Path;

    queue.push_back(Path {
        map: matrix.clone(),
        length: 0,
        current_pos: start_tile,
        current_dir: (0, 1),
    });
    // longest = queue[0].clone();
    let mut longest_length = 0;

    while let Some(current_path) = queue.pop_front() {
        // current_path.visualize_path();
        let Path {
            map,
            length,
            current_pos,
            current_dir,
        } = current_path;

        if current_pos == goal {
            if length > longest_length {
                longest_length = length;
            }
            continue;
        }
        let (x, y) = current_pos;
        let (vx, vy) = current_dir;

        if map[current_pos.1][current_pos.0] == 'O' {
            continue;
        }
        // let mut new_nodes = Vec::<Node>::new();
        {
            // continue straight
            let nx = x as isize + vx;
            let ny = y as isize + vy;
            if nx >= 0
                && nx < matrix[0].len() as isize
                && ny >= 0
                && ny < matrix.len() as isize
                && matrix[ny as usize][nx as usize] != '#'
                && (matrix[y][x] == '.' || slope_to_vector(&matrix[y][x]) == current_dir)
            {
                // println!("Moving straight");
                let mut new_map = map.clone();
                new_map[current_pos.1][current_pos.0] = 'O';
                queue.push_back(Path {
                    map: new_map,
                    length: length + 1,
                    current_pos: (nx as usize, ny as usize),
                    current_dir: current_dir,
                });
            }
        }

        {
            // turn left
            let (dx, dy) = match current_dir {
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                _ => panic!("Invalid direction"),
            };
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0
                && nx < matrix[0].len() as isize
                && ny >= 0
                && ny < matrix.len() as isize
                && matrix[ny as usize][nx as usize] != '#'
                && (matrix[y][x] == '.' || slope_to_vector(&matrix[y][x]) == (dx, dy))
            {
                // println!("Turning left");
                let mut new_map = map.clone();
                new_map[current_pos.1][current_pos.0] = 'O';
                queue.push_back(Path {
                    map: new_map,
                    length: length + 1,
                    current_pos: (nx as usize, ny as usize),
                    current_dir: (dx, dy),
                });
            }
        }

        // turn right
        {
            let (dx, dy) = match current_dir {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => panic!("Invalid direction"),
            };
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0
                && nx < matrix[0].len() as isize
                && ny >= 0
                && ny < matrix.len() as isize
                && matrix[ny as usize][nx as usize] != '#'
                && (matrix[y][x] == '.' || slope_to_vector(&matrix[y][x]) == (dx, dy))
            {
                // println!("Turning right");
                let mut new_map = map.clone();
                new_map[current_pos.1][current_pos.0] = 'O';
                queue.push_back(Path {
                    map: new_map,
                    length: length + 1,
                    current_pos: (nx as usize, ny as usize),
                    current_dir: (dx, dy),
                });
            }
        }
    }

    let sum = longest_length;
    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

fn part_2(reader: &str) {
    let start = Instant::now();
    let lines = reader.lines();
    let matrix = lines
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_tile = (1, 0); // (x, y)
    let goal = (matrix[0].len() - 2, matrix.len() - 1);

    let mut graph_nodes = Vec::new();
    // Getting all nodes in graph (more than 2 neighbors + ends)
    {
        graph_nodes.push(start_tile);
        graph_nodes.push(goal);

        for y in 0..matrix.len() {
            for x in 0..matrix[0].len() {
                if matrix[y][x] == '#' {
                    continue;
                }
                let neighbors = get_valid_neighbors(&matrix, (x, y));
                if neighbors.len() < 3 {
                    continue;
                }
                graph_nodes.push((x, y));
            }
        }
    }

    // Map of nodes to neighboring nodes and their distance
    let mut graph: FxHashMap<(usize, usize), FxHashMap<(usize, usize), usize>> =
        FxHashMap::from_iter(
            graph_nodes
                .iter()
                .map(|&node| (node, FxHashMap::<(usize, usize), usize>::default())),
        );

    // Get length of edges
    {
        for node in &graph_nodes {
            // Path find to nearest nodes
            let mut stack = Vec::new();
            let mut visited = FxHashSet::<(usize, usize)>::default();
            stack.push((0, *node));

            while let Some((length, current_coord)) = stack.pop() {
                visited.insert(current_coord);

                if length > 0 && graph_nodes.contains(&current_coord) {
                    graph.get_mut(node).unwrap().insert(current_coord, length);
                    continue;
                }

                for neighbor in get_valid_neighbors(&matrix, current_coord) {
                    if !visited.contains(&neighbor) {
                        stack.push((length + 1, neighbor));
                    }
                }
            }
        }
    }

    let mut longest_length = 0;

    // Search for longest path
    {
        let mut stack = Vec::<((usize, usize), usize, Vec<(usize, usize)>)>::new();
        stack.push((start_tile, 0, Vec::new()));

        while let Some((current_pos, length, visited)) = stack.pop() {
            if current_pos == goal {
                longest_length = length.max(longest_length);
                continue;
            }

            let neighbors = graph.get(&current_pos).unwrap().keys();
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    let mut new_visited = visited.clone();
                    new_visited.push(current_pos);
                    let delta = graph.get(&current_pos).unwrap().get(neighbor).unwrap();
                    stack.push((*neighbor, length + delta, new_visited));
                }
            }
        }
    }
    println!("Part 2: {} in {:?}", longest_length, start.elapsed());
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Node {
    g: usize,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
}

#[derive(Clone, Debug)]
struct Path {
    map: Vec<Vec<char>>,
    length: usize,
    current_pos: (usize, usize),
    current_dir: (isize, isize),
}

fn slope_to_vector(slope: &char) -> (isize, isize) {
    match slope {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        '.' => (0, 0),
        _ => panic!("Invalid slope"),
    }
}

fn get_valid_neighbors(matrix: &Vec<Vec<char>>, coords: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::<(usize, usize)>::new();
    let (x, y) = coords;
    if x > 0 && matrix[y][x - 1] != '#' {
        neighbors.push((x - 1, y));
    }
    if x < matrix[0].len() - 1 && matrix[y][x + 1] != '#' {
        neighbors.push((x + 1, y));
    }
    if y > 0 && matrix[y - 1][x] != '#' {
        neighbors.push((x, y - 1));
    }
    if y < matrix.len() - 1 && matrix[y + 1][x] != '#' {
        neighbors.push((x, y + 1));
    }
    neighbors
}
