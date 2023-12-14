use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    // let reader = read_to_string("./inputs/test14A.txt").unwrap();
    let reader = read_to_string("./inputs/day14.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let start = Instant::now();

    let mut board = reader
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    tilt_north(&mut board);

    let sum = calculate_load(&board);

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

fn part_2(reader: &str) {
    let start = Instant::now();

    let mut map = HashMap::new();

    let mut board = reader
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;

    // Run until we find a cycle
    for i in 0..1_000_000_000 as usize {
        cycle(&mut board);
        // Check if we have seen this board before
        if let Some(j) = map.insert(board.clone(), i) {
            // Get board at 1bil based on cycle
            let end = i;
            let begin = j;
            let period = end - begin;
            let index = (1_000_000_000 - begin - 1) % period;
            let new_board = map.iter().find(|(_, &v)| v == begin + index).unwrap().0;
            sum = calculate_load(new_board);
            break;
        }
    }

    println!("Part 2: {} in {:?}", sum, start.elapsed());
}

fn cycle(board: &mut Vec<Vec<char>>) {
    tilt_north(board);
    tilt_west(board);
    tilt_south(board);
    tilt_east(board);
}

fn tilt_north(board: &mut Vec<Vec<char>>) {
    let lx = board[0].len();
    let ly = board.len();
    for x in 0..lx {
        // By column
        for starty in 1..=ly - 1 {
            // For each row
            let mut y = starty;
            while y >= 1 && board[y][x] == 'O' && board[y - 1][x] == '.' {
                // Swap rock to empty spot effectively moving it north
                board[y][x] = '.';
                board[y - 1][x] = 'O';
                y -= 1;
            }
        }
    }
}

fn tilt_west(board: &mut Vec<Vec<char>>) {
    let lx = board[0].len();
    let ly = board.len();
    // By row
    for y in 0..ly {
        for startx in 1..=lx - 1 {
            // For each row
            let mut x = startx;
            while x >= 1 && board[y][x] == 'O' && board[y][x - 1] == '.' {
                // Swap rock to empty spot effectively moving it west
                board[y][x] = '.';
                board[y][x - 1] = 'O';
                x -= 1;
            }
        }
    }
}

fn tilt_south(board: &mut Vec<Vec<char>>) {
    let lx = board[0].len();
    let ly = board.len();
    for x in 0..lx {
        // By column
        for starty in (1..=ly - 1).rev() {
            // For each row
            let mut y = starty;
            while y < ly && board[ly - y - 1][x] == 'O' && board[ly - y][x] == '.' {
                // Swap rock to empty spot effectively moving it north
                board[ly - y - 1][x] = '.';
                board[ly - y][x] = 'O';
                y += 1;
            }
        }
    }
}

fn tilt_east(board: &mut Vec<Vec<char>>) {
    let lx = board[0].len();
    let ly = board.len();
    // By row
    for y in 0..ly {
        for startx in (1..=lx - 1).rev() {
            // For each row
            let mut x = startx;
            while x < lx && board[y][lx - x - 1] == 'O' && board[y][lx - x] == '.' {
                // Swap rock to empty spot effectively moving it west
                board[y][lx - x - 1] = '.';
                board[y][lx - x] = 'O';
                x += 1;
            }
        }
    }
}

fn calculate_load(board: &Vec<Vec<char>>) -> usize {
    let ly = board.len();
    let load = board
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .filter(|c| **c == 'O')
                .map(|_| ly - y)
                .sum::<usize>()
        })
        .sum::<usize>();
    load
}

fn _print_board(board: &Vec<Vec<char>>) {
    for row in board {
        println!("{}", row.iter().collect::<String>());
    }
}
