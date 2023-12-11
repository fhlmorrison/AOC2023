use std::{collections::HashSet, fs::read_to_string, vec};
fn main() {
    // let reader = read_to_string("./inputs/test11A.txt").unwrap();
    let reader = read_to_string("./inputs/day11.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

// Creating expanded grid
fn part_1(reader: &str) {
    let lines = reader.lines();

    let grid = lines
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut new_rows = Vec::new();

    for row in grid {
        if row.iter().all(|c| !c) {
            new_rows.push(row.clone());
        }
        new_rows.push(row);
    }

    let mut new_cols = vec![Vec::<bool>::new(); new_rows.len()];
    for i in 0..new_rows[0].len() {
        if new_rows.iter().all(|row| !row[i]) {
            for j in 0..new_rows.len() {
                new_cols[j].push(false);
            }
        }
        for (j, row) in new_rows.iter().enumerate() {
            new_cols[j].push(row[i]);
        }
    }

    let galaxies = new_cols
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c)
                .map(move |(j, _)| (i, j))
        })
        .collect::<Vec<_>>();

    let sum = galaxies.iter().enumerate().fold(0, |acc, val| {
        let (i, (x, y)) = val;
        acc + galaxies[i + 1..]
            .iter()
            .fold(0, |acc, (x2, y2)| acc + x.abs_diff(*x2) + y.abs_diff(*y2))
    });

    println!("Part 1: {}", sum);
}

// Tracking expanded rows and columns
fn part_2(reader: &str) {
    let lines = reader.lines();

    let mut expanded_rows = HashSet::<usize>::new();
    let mut expanded_cols = HashSet::<usize>::new();

    let grid = lines
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    for (i, row) in grid.iter().enumerate() {
        if row.iter().all(|c| !c) {
            expanded_rows.insert(i);
        }
    }

    for i in 0..grid[0].len() {
        if grid.iter().all(|row| !row[i]) {
            expanded_cols.insert(i);
        }
    }

    let galaxies = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c)
                .map(move |(j, _)| (i, j))
        })
        .collect::<Vec<_>>();

    let mut millions = 0;

    let sum = galaxies.iter().enumerate().fold(0, |acc, val| {
        let (i, (x, y)) = val;
        acc + galaxies[i + 1..].iter().fold(0, |acc, (x2, y2)| {
            let rangex = if x < x2 { *x..*x2 } else { *x2..*x };
            let rangey = if y < y2 { *y..*y2 } else { *y2..*y };
            let dx = rangex.fold(0, |acc, row| {
                if expanded_rows.contains(&row) {
                    millions += 1;
                    acc
                } else {
                    acc + 1
                }
            });
            let dy = rangey.fold(0, |acc, row| {
                if expanded_cols.contains(&row) {
                    millions += 1;
                    acc
                } else {
                    acc + 1
                }
            });
            acc + dx + dy
        })
    });

    // Plug output into calculator for answer
    println!("Part 2: {}000000 + {}", millions, sum);
}
