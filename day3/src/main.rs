use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let reader = read_to_string("./inputs/day3.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let matrix = reader
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;

    let mut matrix_iter = matrix.iter().enumerate();

    // Iterate over the matrix
    while let Some((row, line)) = matrix_iter.next() {
        let mut line_iter = line.iter().enumerate();
        // If a gear is found, expand the number and add it to the sum
        while let Some((col, c)) = line_iter.next() {
            if c.is_digit(10) {
                if search_symbol(&matrix, row, col) {
                    sum += expand_num(&matrix, row, col);
                    // Skip the rest of the number to avoid adding it twice
                    while line_iter.next().map_or(false, |(_, c)| c.is_digit(10)) {}
                }
            }
        }
    }

    println!("Part 1: {}", sum);
}

// Search for a symbol around the given position
fn search_symbol(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    for delta_row in [-1, 0, 1] {
        for delta_col in [-1, 0, 1] {
            if delta_row == 0 && delta_col == 0 {
                continue;
            }
            let new_row = row as i32 + delta_row;
            let new_col = col as i32 + delta_col;

            if new_row < 0 || new_col < 0 {
                continue;
            }

            // Only check valid positions
            if let Some(Some(search_element)) = matrix
                .get(new_row as usize)
                .map(|line| line.get(new_col as usize))
            {
                // Symbol is just not a digit and not a dot
                if !search_element.is_digit(10) && search_element != &'.' {
                    return true;
                }
            }
        }
    }
    false
}

// Expand the number at the given position
fn expand_num(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut start = col;
    let mut end = col;

    // Search for the start of the number
    while start > 0 && matrix[row][start - 1].is_digit(10) {
        start -= 1;
    }

    // Search for the end of the number
    while end < matrix[row].len() - 1 && matrix[row][end + 1].is_digit(10) {
        end += 1;
    }
    let num_str = matrix[row][start..=end].iter().collect::<String>();
    // println!("Found number: {} at {row}, {start}..{end}", num_str);
    num_str.parse::<u32>().unwrap()
}

fn part_2(reader: &str) {
    let matrix = reader
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut gears_map: HashMap<(usize, usize), (u8, u32)> = HashMap::new();

    let mut matrix_iter = matrix.iter().enumerate();

    while let Some((row, line)) = matrix_iter.next() {
        let mut line_iter = line.iter().enumerate();
        while let Some((col, c)) = line_iter.next() {
            if c.is_digit(10) {
                if let Some(coords) = search_gear(&matrix, row, col) {
                    let num = expand_num(&matrix, row, col);
                    if let Some(old_v) = gears_map.insert(coords, (1, num)) {
                        gears_map.insert(coords, (old_v.0 + 1, old_v.1 * num));
                    }
                    while line_iter.next().map_or(false, |(_, c)| c.is_digit(10)) {}
                }
            }
        }
    }
    let sum = gears_map.iter().fold(0, |acc, (_, v)| {
        // println!("{} {}", v.0, v.1);
        acc + if v.0 == 2 { v.1 } else { 0 }
    });

    println!("Part 2: {}", sum);
}

// Search for a '*' around the given position and return the position of the '*'
fn search_gear(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> Option<(usize, usize)> {
    for delta_row in [-1, 0, 1] {
        for delta_col in [-1, 0, 1] {
            if delta_row == 0 && delta_col == 0 {
                continue;
            }
            let new_row = row as i32 + delta_row;
            let new_col = col as i32 + delta_col;

            if new_row < 0 || new_col < 0 {
                continue;
            }

            if let Some(Some(search_element)) = matrix
                .get(new_row as usize)
                .map(|line| line.get(new_col as usize))
            {
                if search_element == &'*' {
                    // println!("Found * at {}, {}", new_row, new_col);
                    return Some((new_row as usize, new_col as usize));
                }
            }
        }
    }
    None
}
