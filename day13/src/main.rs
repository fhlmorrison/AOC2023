use rayon::prelude::*;
use std::cmp::min;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    // let reader = read_to_string("./inputs/test13A.txt").unwrap();
    let reader = read_to_string("./inputs/day13.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let start = Instant::now();
    let patterns = reader.split("\r\n\r\n");

    let sum = patterns.fold(0, |acc, pattern| acc + get_pattern_value(pattern));

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

fn part_2(reader: &str) {
    let start = Instant::now();
    let patterns = reader.split("\r\n\r\n");

    let sum = patterns.fold(0, |acc, pattern| acc + get_pattern_value2(pattern));

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

fn get_pattern_value(pattern: &str) -> usize {
    let matrix = pattern
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Check each row for symmetry
    'outer: for row in 1..matrix.len() {
        // Check each row pair for symmetry
        for drow in 1..min(row, matrix.len() - row) + 1 {
            // Check each column for symmetry
            for col in 0..matrix[0].len() {
                if matrix[row + drow - 1][col] != matrix[row - drow][col] {
                    continue 'outer;
                }
            }
        }
        return row * 100;
    }

    // Instead of doing new stuff for columns, we just transpose the matrix
    let matrix = transpose(matrix);

    // Check each column for symmetry
    'outer: for row in 1..matrix.len() {
        // Check each row pair for symmetry
        for drow in 1..min(row, matrix.len() - row) + 1 {
            // Check each column for symmetry
            for col in 0..matrix[0].len() {
                if matrix[row + drow - 1][col] != matrix[row - drow][col] {
                    continue 'outer;
                }
            }
        }
        return row;
    }

    0
}

fn get_pattern_value2(pattern: &str) -> usize {
    let matrix = pattern
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Check each row for symmetry
    'outer: for row in 1..matrix.len() {
        let mut count = 0;
        // Check each row pair for symmetry
        for drow in 1..min(row, matrix.len() - row) + 1 {
            // Check each column for symmetry
            for col in 0..matrix[0].len() {
                if matrix[row + drow - 1][col] != matrix[row - drow][col] {
                    if count > 0 {
                        continue 'outer;
                    }
                    count += 1;
                }
            }
        }
        if count == 1 {
            return row * 100;
        }
    }

    // Instead of doing new stuff for columns, we just transpose the matrix
    let matrix = transpose(matrix);

    // Check each column for symmetry
    'outer: for row in 1..matrix.len() {
        let mut count = 0;
        // Check each row pair for symmetry
        for drow in 1..min(row, matrix.len() - row) + 1 {
            for col in 0..matrix[0].len() {
                if matrix[row + drow - 1][col] != matrix[row - drow][col] {
                    if count > 0 {
                        // Allows for only 1 smudge
                        continue 'outer;
                    }
                    count += 1;
                }
            }
        }
        if count == 1 {
            // Success only if there is 1 smudge
            return row;
        }
    }
    0
}

// ty rust forums for this transpose <3
pub fn transpose<T: Send + Sync + Copy>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    (0..len)
        .into_par_iter()
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}
