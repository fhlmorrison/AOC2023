use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    // let reader = read_to_string("./inputs/test18A.txt").unwrap();
    let reader = read_to_string("./inputs/day18.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let start = Instant::now();
    let lines = reader.lines();

    let mut visited = HashSet::new();

    let mut current = (0isize, 0isize);
    visited.insert(current);

    for line in lines {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let direction = split[0];
        let distance = split[1].parse::<i32>().unwrap();

        for _ in 0..distance {
            match direction {
                "R" => current.0 += 1,
                "L" => current.0 -= 1,
                "U" => current.1 += 1,
                "D" => current.1 -= 1,
                _ => panic!("Unknown direction"),
            }
            visited.insert(current);
        }
    }

    let min_row = visited.iter().map(|(_, y)| y).min().unwrap();
    let max_row = visited.iter().map(|(_, y)| y).max().unwrap();
    let min_col = visited.iter().map(|(x, _)| x).min().unwrap();
    let max_col = visited.iter().map(|(x, _)| x).max().unwrap();

    let x_size = (max_col - min_col + 1) as usize;
    let y_size = (max_row - min_row + 1) as usize;

    let mut map: Vec<Vec<bool>> = vec![vec![false; x_size]; y_size];

    for y in *min_row..=*max_row {
        // print!("{y: >2} ");
        for x in *min_col..=*max_col {
            if visited.contains(&(x, y)) {
                map[(y - min_row) as usize][(x - min_col) as usize] = true;
                // print!("#");
            } else {
                // print!(".");
            }
        }
        // println!();
    }
    // print!("   ");
    // for x in 0..x_size {
    //     print!("{x}");
    // }
    // println!();

    let trench_area = map.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, cell| acc + if *cell { 1 } else { 0 })
    });
    // println!("Trench area: {}", trench_area);
    let mut edges = VecDeque::new();

    for i in 0..y_size {
        edges.push_back((i, 0));
        edges.push_back((i, x_size - 1));
    }
    for i in 0..x_size {
        edges.push_back((0, i));
        edges.push_back((y_size - 1, i));
    }

    let mut sum = 0;

    while let Some(tile) = edges.pop_front() {
        let (y, x) = tile;
        if map[y][x] {
            continue;
        }
        map[y][x] = true;
        sum += 1;
        if y > 0 {
            edges.push_back((y - 1, x));
        }
        if y < y_size - 1 {
            edges.push_back((y + 1, x));
        }
        if x > 0 {
            edges.push_back((y, x - 1));
        }
        if x < x_size - 1 {
            edges.push_back((y, x + 1));
        }
    }

    let sum = x_size * y_size - sum;

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

fn part_2(reader: &str) {
    let start = Instant::now();
    let lines = reader.lines();

    let mut vertices = Vec::new();
    let mut boundary_points = 0;

    let mut visited = HashSet::new();

    let mut current = (0isize, 0isize);
    visited.insert(current);

    for line in lines {
        let split = line.split_whitespace().collect::<Vec<&str>>();

        let hex = split[2]
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(")")
            .unwrap();

        let direction = match hex.chars().last() {
            Some('0') => "R",
            Some('1') => "D",
            Some('2') => "L",
            Some('3') => "U",
            _ => panic!("Unknown direction"),
        };
        let distance = isize::from_str_radix(&hex[..hex.len() - 1], 16).unwrap();

        vertices.push(current);
        let delta = delta_from_direction(direction);
        current.0 += delta.0 * distance;
        current.1 += delta.1 * distance;
        boundary_points += distance;
    }

    let area = shoelace_formula(&vertices);

    let sum = picks_theorem(&area, &boundary_points);
    println!("Part 2: {} in {:?}", sum, start.elapsed());
}

fn shoelace_formula(vertices: &Vec<(isize, isize)>) -> isize {
    let mut area = 0;

    for i in 0..vertices.len() {
        let (x1, y1) = vertices[i];
        let (x2, y2) = if i == vertices.len() - 1 {
            vertices[0]
        } else {
            vertices[i + 1]
        };

        area += x1 * y2 - x2 * y1;
    }
    area.abs() / 2
}

fn delta_from_direction(direction: &str) -> (isize, isize) {
    match direction {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("Unknown direction"),
    }
}

fn picks_theorem(area: &isize, boundary_points: &isize) -> isize {
    (area - boundary_points / 2 + 1) + boundary_points
}
