use colored::Colorize; // For pretty printing the map
use core::fmt; // For pretty printing the map
use std::{collections::VecDeque, fs::read_to_string};
fn main() {
    // let reader = read_to_string("./inputs/test10A.txt").unwrap();
    let reader = read_to_string("./inputs/day10.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

// For pretty printing the map
impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::NS => write!(f, "{}", "║"),
            Pipe::EW => write!(f, "{}", "═"),
            Pipe::NE => write!(f, "{}", "╚"),
            Pipe::NW => write!(f, "{}", "╝"),
            Pipe::SW => write!(f, "{}", "╗"),
            Pipe::SE => write!(f, "{}", "╔"),
            Pipe::Ground => write!(f, "{}", "."),
            Pipe::Start => write!(f, "{}", "╳"),
        }
    }
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => Pipe::Ground,
        }
    }
}

fn part_1(reader: &str) {
    let lines = reader.lines();

    let matrix = lines
        .map(|line| line.chars().map(|c| Pipe::from(c)).collect::<Vec<Pipe>>())
        .collect::<Vec<Vec<Pipe>>>();

    let start = matrix
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, pipe)| {
                if *pipe == Pipe::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let (x, y) = start;

    // find a connecting pipe
    for i in 0..4 {
        let (dx, dy) = match i {
            0 => (0, 1),
            1 => (0, -1),
            2 => (1, 0),
            3 => (-1, 0),
            _ => panic!("Invalid direction"),
        };
        let (mut nx, mut ny) = (x as isize + dx, y as isize + dy);
        if nx < 0 || ny < 0 {
            continue;
        }
        let pipe = matrix.get(ny as usize).and_then(|row| row.get(nx as usize));
        if pipe.is_none() {
            continue;
        }
        let pipe = pipe.unwrap();
        if pipe == &Pipe::Ground {
            continue;
        }
        let direction = match i {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            3 => Direction::West,
            _ => panic!("Invalid direction"),
        };

        let mut distance = 0;
        let mut direction_in = direction;

        if !is_valid_pipe(&direction.clone(), pipe) {
            continue;
        }

        loop {
            let x = nx as usize;
            let y = ny as usize;
            let pipe = matrix.get(y).and_then(|row| row.get(x));
            if pipe.is_none() {
                panic!("Pipe not found");
            }
            let pipe = pipe.unwrap();

            if pipe == &Pipe::Ground {
                panic!("Pipe is ground");
            }
            if pipe == &Pipe::Start {
                break;
            }
            // Find next connecting pipes
            if let Some(direction_out) = map_direction(&direction_in, pipe) {
                let (dx, dy) = direction_to_offsets(&direction_out);

                (nx, ny) = (x as isize + dx, y as isize + dy);

                if nx < 0 || ny < 0 {
                    panic!("Pipe not found");
                }

                let new_direction = flip_direction(&direction_out);
                direction_in = new_direction;
                distance += 1;
                continue;
            } else {
                panic!("Pipe not found");
            }
        }
        println!("Part 1: {}", distance / 2 + distance % 2);
        break;
    }
}

fn part_2(reader: &str) {
    let lines = reader.lines();

    let matrix = lines
        .map(|line| line.chars().map(|c| Pipe::from(c)).collect::<Vec<Pipe>>())
        .collect::<Vec<Vec<Pipe>>>();

    let mut map_matrix = vec![vec![0; matrix[0].len()]; matrix.len()];

    let start = matrix
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, pipe)| {
                if *pipe == Pipe::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let (x, y) = start;
    map_matrix[y][x] = 2;

    // find a connecting pipe
    for i in 0..4 {
        let (dx, dy) = match i {
            0 => (0, 1),
            1 => (0, -1),
            2 => (1, 0),
            3 => (-1, 0),
            _ => panic!("Invalid direction"),
        };
        let (mut nx, mut ny) = (x as isize + dx, y as isize + dy);
        if nx < 0 || ny < 0 {
            continue;
        }
        let pipe = matrix.get(ny as usize).and_then(|row| row.get(nx as usize));
        if pipe.is_none() {
            continue;
        }
        let pipe = pipe.unwrap();
        if pipe == &Pipe::Ground {
            continue;
        }
        let direction = match i {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            3 => Direction::West,
            _ => panic!("Invalid direction"),
        };

        let mut distance = 0;
        let mut direction_in = direction;

        if !is_valid_pipe(&direction.clone(), pipe) {
            continue;
        }

        // Traverse loop
        loop {
            let x = nx as usize;
            let y = ny as usize;
            // Mark tiles that are a part of loop
            map_matrix[y][x] = 2;
            let pipe = matrix.get(y).and_then(|row| row.get(x));
            if pipe.is_none() {
                panic!("Pipe not found");
            }
            let pipe = pipe.unwrap();

            if pipe == &Pipe::Ground {
                panic!("Pipe is ground");
            }
            if pipe == &Pipe::Start {
                break;
            }
            // Find next connecting pipe
            if let Some(direction_out) = map_direction(&direction_in, pipe) {
                let (dx, dy) = direction_to_offsets(&direction_out);

                (nx, ny) = (x as isize + dx, y as isize + dy);

                if nx < 0 || ny < 0 {
                    panic!("Pipe not found");
                }

                let (left, _right) = split_sides(&direction_in, &direction_out);

                // Mark tiles on left side of loop as you traverse
                left.iter().for_each(|dir| {
                    let (dx, dy) = direction_to_offsets(&dir);
                    let (newx, newy) = (x as isize + dx, y as isize + dy);
                    if newx < 0
                        || newy < 0
                        || newx >= matrix[0].len() as isize
                        || newy >= matrix.len() as isize
                    {
                        return;
                    }
                    if map_matrix[newy as usize][newx as usize] == 2 {
                        return;
                    }
                    map_matrix[newy as usize][newx as usize] = 1;
                });

                let new_direction = flip_direction(&direction_out);
                direction_in = new_direction;
                distance += 1;
                continue;
            } else {
                panic!("Pipe not found");
            }
        }
        println!("Distance: {}", distance);
        break;
    }

    let sum_twos = map_matrix.iter().fold(0, |acc, row| {
        row.iter()
            .fold(acc, |acc, pipe| if pipe == &2 { acc + 1 } else { acc })
    });

    find_nest(&matrix, &mut map_matrix);

    // let sum = 0;
}

fn map_direction(direction_in: &Direction, pipe: &Pipe) -> Option<Direction> {
    match pipe {
        Pipe::NS => match direction_in {
            Direction::North => Some(Direction::South),
            Direction::South => Some(Direction::North),
            _ => None,
        },
        Pipe::EW => match direction_in {
            Direction::East => Some(Direction::West),
            Direction::West => Some(Direction::East),
            _ => None,
        },
        Pipe::NE => match direction_in {
            Direction::North => Some(Direction::East),
            Direction::East => Some(Direction::North),
            _ => None,
        },
        Pipe::NW => match direction_in {
            Direction::North => Some(Direction::West),
            Direction::West => Some(Direction::North),
            _ => None,
        },
        Pipe::SW => match direction_in {
            Direction::South => Some(Direction::West),
            Direction::West => Some(Direction::South),
            _ => None,
        },
        Pipe::SE => match direction_in {
            Direction::South => Some(Direction::East),
            Direction::East => Some(Direction::South),
            _ => None,
        },
        Pipe::Ground => None,
        _ => None,
    }
}

fn direction_to_offsets(direction: &Direction) -> (isize, isize) {
    match direction {
        Direction::North => (0, -1),
        Direction::South => (0, 1),
        Direction::East => (1, 0),
        Direction::West => (-1, 0),
    }
}

fn flip_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

fn is_valid_pipe(direction: &Direction, pipe: &Pipe) -> bool {
    match pipe {
        Pipe::NS => match direction {
            Direction::North => true,
            Direction::South => true,
            _ => false,
        },
        Pipe::EW => match direction {
            Direction::East => true,
            Direction::West => true,
            _ => false,
        },
        Pipe::NE => match direction {
            Direction::North => true,
            Direction::East => true,
            _ => false,
        },
        Pipe::NW => match direction {
            Direction::North => true,
            Direction::West => true,
            _ => false,
        },
        Pipe::SW => match direction {
            Direction::South => true,
            Direction::West => true,
            _ => false,
        },
        Pipe::SE => match direction {
            Direction::South => true,
            Direction::East => true,
            _ => false,
        },
        Pipe::Ground => false,
        _ => false,
    }
}

fn find_nest(original_matrix: &Vec<Vec<Pipe>>, map_matrix: &mut Vec<Vec<i32>>) {
    // Expand area from all cells marked as 1 (left side of loop)

    let mut queue = VecDeque::new();

    for i in 0..map_matrix[0].len() {
        for j in 0..map_matrix.len() {
            if map_matrix[j][i] == 1 {
                let neighbours = get_neighbours(i, j, original_matrix);
                neighbours.iter().for_each(|(nx, ny)| {
                    if map_matrix[*ny][*nx] == 0 {
                        queue.push_back((*nx, *ny));
                    }
                });
            }
        }
    }

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if map_matrix[y][x] != 0 {
            continue;
        }
        map_matrix[y][x] = 1;
        // Check all neighbours
        // if neighbour is 0, add to queue
        let neighbours = get_neighbours(x, y, original_matrix);
        neighbours.iter().for_each(|(nx, ny)| {
            if map_matrix[*ny][*nx] == 0 {
                queue.push_back((*nx, *ny));
            }
        });
    }

    let left = map_matrix.iter().enumerate().fold(0, |acc, (y, row)| {
        println!();
        acc + row.iter().enumerate().fold(0, |acc, (x, pipe)| {
            if *pipe == 0 {
                print!("{}", original_matrix[y][x].to_string().red());
                acc + 1
            } else {
                if *pipe == 2 {
                    print!("{}", original_matrix[y][x].to_string().green());
                } else {
                    print!("{}", original_matrix[y][x].to_string());
                }
                acc
            }
        })
    });

    let right = map_matrix.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row.iter().enumerate().fold(
            0,
            |acc, (x, pipe)| {
                if *pipe == 1 {
                    acc + 1
                } else {
                    acc
                }
            },
        )
    });
    println!();
    let pipes = map_matrix.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, pipe| if *pipe == 2 { acc + 1 } else { acc })
    });
    println!("Part 1: {}", pipes / 2 + pipes % 2);
    println!("Part 2: {} red, {} white, {} green", left, right, pipes);
}

fn get_neighbours(x: usize, y: usize, matrix: &Vec<Vec<Pipe>>) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if x < matrix[0].len() - 1 {
        neighbours.push((x + 1, y));
    }
    if y < matrix.len() - 1 {
        neighbours.push((x, y + 1));
    }
    if x > 0 && y > 0 {
        neighbours.push((x - 1, y - 1));
    }
    if x < matrix[0].len() - 1 && y > 0 {
        neighbours.push((x + 1, y - 1));
    }
    if x > 0 && y < matrix.len() - 1 {
        neighbours.push((x - 1, y + 1));
    }
    if x < matrix[0].len() - 1 && y < matrix.len() - 1 {
        neighbours.push((x + 1, y + 1));
    }
    neighbours
}

fn split_sides(
    direction_in: &Direction,
    direction_out: &Direction,
) -> (Vec<Direction>, Vec<Direction>) {
    // (left, right)
    match direction_in {
        Direction::North => match direction_out {
            Direction::South => (vec![Direction::East], vec![Direction::West]),
            Direction::East => (vec![], vec![Direction::South, Direction::West]),
            Direction::West => (vec![Direction::South, Direction::East], vec![]),
            _ => panic!("Invalid direction"),
        },
        Direction::South => match direction_out {
            Direction::North => (vec![Direction::West], vec![Direction::East]),
            Direction::East => (vec![Direction::West, Direction::North], vec![]),
            Direction::West => (vec![], vec![Direction::North, Direction::East]),
            _ => panic!("Invalid direction"),
        },
        Direction::East => match direction_out {
            Direction::North => (vec![Direction::South, Direction::West], vec![]),
            Direction::South => (vec![], vec![Direction::North, Direction::West]),
            Direction::West => (vec![Direction::South], vec![Direction::North]),
            _ => panic!("Invalid direction"),
        },
        Direction::West => match direction_out {
            Direction::North => (vec![], vec![Direction::South, Direction::East]),
            Direction::South => (
                vec![Direction::North, Direction::East],
                vec![Direction::South],
            ),
            Direction::East => (vec![Direction::North], vec![Direction::South]),
            _ => panic!("Invalid direction"),
        },
    }
}
