use rayon::prelude::*; // For parallelizing the search
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    // let reader = read_to_string("./inputs/test16A.txt").unwrap();
    let reader = read_to_string("./inputs/day16.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Step {
    x: usize,
    y: usize,
    direction: Direction,
}

fn part_1(reader: &str) {
    let start = Instant::now();
    let matrix = reader
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let start_step = Step {
        x: 0,
        y: 0,
        direction: Direction::Right,
    };

    let sum = count_energized(&matrix, start_step);

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

fn part_2(reader: &str) {
    let start = Instant::now();
    let matrix = reader
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut starting_steps = Vec::<Step>::new();

    let y_len = matrix.len();
    let x_len = matrix[0].len();

    // Seed the starting steps
    for i in 0..x_len {
        starting_steps.push(Step {
            x: i,
            y: 0,
            direction: Direction::Down,
        });
        starting_steps.push(Step {
            x: i,
            y: y_len - 1,
            direction: Direction::Up,
        });
    }

    for i in 0..y_len {
        starting_steps.push(Step {
            x: 0,
            y: i,
            direction: Direction::Right,
        });
        starting_steps.push(Step {
            x: x_len - 1,
            y: i,
            direction: Direction::Left,
        });
    }

    let sum = starting_steps
        .par_iter()
        .map(|step| count_energized(&matrix, *step))
        .max()
        .unwrap();

    println!("Part 2: {} in {:?}", sum, start.elapsed());
}

fn direction_to_coords(direction: Direction) -> (isize, isize) {
    match direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

fn map_direction(direction_in: &Direction, tile: &char) -> (Option<Direction>, Option<Direction>) {
    match tile {
        '-' => match direction_in {
            Direction::Up => (Some(Direction::Left), Some(Direction::Right)),
            Direction::Down => (Some(Direction::Right), Some(Direction::Left)),
            _ => (Some(direction_in.clone()), None),
        },
        '|' => match direction_in {
            Direction::Right => (Some(Direction::Up), Some(Direction::Down)),
            Direction::Left => (Some(Direction::Down), Some(Direction::Up)),
            _ => (Some(direction_in.clone()), None),
        },
        '\\' => match direction_in {
            Direction::Up => (Some(Direction::Left), None),
            Direction::Down => (Some(Direction::Right), None),
            Direction::Left => (Some(Direction::Up), None),
            Direction::Right => (Some(Direction::Down), None),
        },
        '/' => match direction_in {
            Direction::Up => (Some(Direction::Right), None),
            Direction::Down => (Some(Direction::Left), None),
            Direction::Right => (Some(Direction::Up), None),
            Direction::Left => (Some(Direction::Down), None),
        },
        _ => (Some(direction_in.clone()), None),
    }
}

fn count_energized(matrix: &Vec<Vec<char>>, start: Step) -> usize {
    let mut energized = vec![vec![0; matrix[0].len()]; matrix.len()];

    // coords and direction

    let mut queue = VecDeque::new();

    let mut tried = HashSet::<Step>::new();

    // Traverse matrix from top left
    queue.push_back(start);

    while let Some(next_step) = queue.pop_front() {
        if tried.contains(&next_step) {
            continue;
        } else {
            tried.insert(next_step);
        }

        let x = next_step.x;
        let y = next_step.y;
        let direction = &next_step.direction;

        energized[next_step.y][next_step.x] = 1;

        let tile = matrix[y as usize][x as usize];

        let (new_direction, new_direction_2) = map_direction(direction, &tile);

        if let Some(new_direction) = new_direction {
            let (dx, dy) = direction_to_coords(new_direction);
            let next_x = next_step.x as isize + dx;
            let next_y = next_step.y as isize + dy;

            // bounds checks
            if !(next_x < 0
                || next_y < 0
                || next_x >= matrix[0].len() as isize
                || next_y >= matrix.len() as isize)
            {
                queue.push_back(Step {
                    x: next_x as usize,
                    y: next_y as usize,
                    direction: new_direction,
                });
            }
        }
        if let Some(new_direction_2) = new_direction_2 {
            let (dx, dy) = direction_to_coords(new_direction_2);
            let next_x = next_step.x as isize + dx;
            let next_y = next_step.y as isize + dy;

            // bounds checks
            if !(next_x < 0
                || next_y < 0
                || next_x >= matrix[0].len() as isize
                || next_y >= matrix.len() as isize)
            {
                queue.push_back(Step {
                    x: next_x as usize,
                    y: next_y as usize,
                    direction: new_direction_2,
                });
            }
        }
    }

    let sum = energized
        .iter()
        .map(|row| row.iter().sum::<usize>())
        .sum::<usize>();
    sum
}
