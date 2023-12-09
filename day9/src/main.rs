use std::fs::read_to_string;

fn main() {
    // let reader = read_to_string("./inputs/test9A.txt").unwrap();
    let reader = read_to_string("./inputs/day9.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let lines = reader.lines();

    let predictions = lines
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            get_prediction(&nums)
        })
        .collect::<Vec<_>>();

    // println!("Part 1: {:?}", predictions);
    let sum = predictions.iter().sum::<isize>();

    println!("Part 1: {}", sum);
}

fn part_2(reader: &str) {
    let lines = reader.lines();

    let predictions = lines
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            get_back_prediction(&nums)
        })
        .collect::<Vec<_>>();

    // println!("Part 2: {:?}", predictions);
    let sum = predictions.iter().sum::<isize>();

    println!("Part 2: {}", sum);
}

fn get_prediction(input: &Vec<isize>) -> isize {
    let slopes = input.windows(2).map(|vs| vs[1] - vs[0]).collect::<Vec<_>>();

    let last = input.last().unwrap();

    if *last == 0 as isize {
        return 0;
    } else {
        return get_prediction(&slopes) + last;
    }
}

fn get_back_prediction(input: &Vec<isize>) -> isize {
    let slopes = input.windows(2).map(|vs| vs[1] - vs[0]).collect::<Vec<_>>();

    let first = input.first().unwrap();

    if *input.last().unwrap() == 0 as isize {
        return 0;
    } else {
        return first - get_back_prediction(&slopes);
    }
}
