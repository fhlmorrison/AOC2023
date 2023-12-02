use std::{fmt::Display, fs::read_to_string};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self {
            Color::Red => "red",
            Color::Green => "green",
            Color::Blue => "blue",
        };
        write!(f, "{}", color)
    }
}

fn main() {
    let reader = read_to_string("./inputs/day2.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let sum = reader.lines().fold(0, |acc, line| acc + valid_line(line));
    println!("Part 1: {}", sum);
}

fn part_2(reader: &str) {
    let sum = reader.lines().fold(0, |acc, line| acc + power_line(line));
    println!("Part 2: {}", sum);
}

fn power_line(line: &str) -> u32 {
    let mut split = line.split(':');
    let _game = split.next().unwrap();
    let rounds_str = split.next().unwrap();

    let rolls = rounds_str
        .split([',', ';'])
        .map(|s| s.trim_start())
        .collect::<Vec<_>>();

    let parsed_rolls = rolls.into_iter().map(parse_roll);

    let maxes = parsed_rolls.fold((0, 0, 0), |acc, (num, color)| match color {
        Color::Red => (std::cmp::max(acc.0, num), acc.1, acc.2),
        Color::Green => (acc.0, std::cmp::max(acc.1, num), acc.2),
        Color::Blue => (acc.0, acc.1, std::cmp::max(acc.2, num)),
    });
    maxes.0 * maxes.1 * maxes.2
}

fn valid_line(line: &str) -> u32 {
    let mut split = line.split(':');
    let game = split.next().unwrap();
    let rounds_str = split.next().unwrap();

    let id: u32 = game.trim_start_matches("Game ").parse().unwrap();

    let rolls = rounds_str
        .split([',', ';'])
        .map(|s| s.trim_start())
        .collect::<Vec<_>>();

    if rolls.into_iter().all(|roll| is_valid(roll)) {
        id
    } else {
        0
    }
}

fn parse_roll(roll: &str) -> (u32, Color) {
    let mut split = roll.split(' ');
    let num = split.next().unwrap().parse::<u32>().unwrap();
    let sides = match split.next().unwrap() {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => panic!("Invalid color"),
    };
    (num, sides)
}

fn is_valid(roll: &str) -> bool {
    let (num, color) = parse_roll(roll);
    match color {
        Color::Red => {
            if num <= 12 {
                return true;
            }
        }
        Color::Green => {
            if num <= 13 {
                return true;
            }
        }
        Color::Blue => {
            if num <= 14 {
                return true;
            }
        }
    }
    false
}
