use std::fs::read_to_string;

fn main() {
    let reader = read_to_string("./inputs/day4.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let lines = reader.lines();

    let sum = lines.fold(0, |acc, line| {
        let count = match_nums(line);
        if count == 0 {
            return acc;
        }
        acc + 2u32.pow(count as u32 - 1)
    });

    println!("Part 1: {}", sum);
}

fn part_2(reader: &str) {
    let lines: Vec<_> = reader.lines().collect();
    // row of Cards to check
    let mut instances: Vec<usize> = vec![1; lines.len()];
    let mut sum = 0;

    for i in 0..lines.len() {
        let curr_instances = instances[i];
        let count = match_nums(lines[i]);
        sum += curr_instances;
        for delta in 1..=count {
            instances[i + delta] += curr_instances;
        }
    }

    println!("Part 2: {}", sum);
}

fn match_nums(line: &str) -> usize {
    let split1: Vec<_> = line.split(':').collect();
    let nums: Vec<_> = split1
        .last()
        .unwrap()
        .split('|')
        .map(|s| s.trim())
        .collect();

    let winning_nums: Vec<_> = nums.first().unwrap().split_whitespace().collect();
    let my_nums: Vec<_> = nums.last().unwrap().split_whitespace().collect();

    let count = my_nums.iter().filter(|&x| winning_nums.contains(x)).count();

    // println!("{}", inter);
    count
}
