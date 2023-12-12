use std::time::{Duration, Instant};
use std::{collections::VecDeque, fs::read_to_string};
fn main() {
    // let reader = read_to_string("./inputs/test12A.txt").unwrap();
    let reader = read_to_string("./inputs/day12.txt").unwrap();

    let start = Instant::now();
    part_1(&reader);
    println!("Part 1 took: {:?}", Instant::now() - start);
    let start = Instant::now();
    part_2(&reader);
    println!("Part 1 took: {:?}", Instant::now() - start);
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl From<&str> for Record {
    fn from(s: &str) -> Self {
        let split = s.split_whitespace().collect::<Vec<&str>>();

        let springs = split[0]
            .chars()
            .map(|c| Spring::from(c))
            .collect::<Vec<Spring>>();

        let groups = split[1]
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Record { springs, groups }
    }
}

impl Record {
    fn validate_group_prefix(&self, prefix: &Vec<usize>) -> bool {
        if prefix.len() > self.groups.len() {
            return false;
        }
        for i in 0..prefix.len() {
            if prefix[i] > self.groups[i] {
                return false;
            }
        }

        true
    }

    fn get_variant_count(&self) -> usize {
        let mut queue = VecDeque::<Vec<Spring>>::new();
        queue.push_back(self.springs.clone());

        let mut sum = 0;

        while let Some(mut curr) = queue.pop_back() {
            if let Some((i, _)) = curr
                .iter()
                .enumerate()
                .find(|(_, s)| *s == &Spring::Unknown)
            {
                curr[i] = Spring::Operational;
                let groups = get_groups(&curr);
                if self.validate_group_prefix(&groups) {
                    queue.push_back(curr.clone());
                }
                curr[i] = Spring::Damaged;
                let groups = get_groups(&curr);
                if self.validate_group_prefix(&groups) {
                    queue.push_back(curr.clone());
                }
            } else {
                let groups = get_groups(&curr);
                if groups == self.groups {
                    sum += 1;
                }
            }
        }

        sum
    }
}

fn get_groups(springs: &Vec<Spring>) -> Vec<usize> {
    let mut groups = vec![];
    let mut group = 0;
    for spring in springs {
        match spring {
            Spring::Operational => {
                if group > 0 {
                    groups.push(group);
                    group = 0;
                }
            }
            Spring::Damaged => {
                group += 1;
            }
            Spring::Unknown => {
                if group > 0 {
                    groups.push(group);
                }
                return groups; // Return early
            }
        }
    }
    if group > 0 {
        groups.push(group);
    }

    // let igroups = springs
    //     .split(|c| c == &Spring::Operational)
    //     .map(|s| s.len())
    //     .filter(|s| *s > 0)
    //     .collect::<Vec<_>>();

    groups
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("Invalid char: {}", c),
        }
    }
}

// Creating expanded grid
fn part_1(reader: &str) {
    let lines = reader.lines();

    let records = lines.map(|line| Record::from(line)).collect::<Vec<_>>();

    // records.iter().for_each(|r| println!("{:?}", r));

    // records
    //     .iter()
    //     .for_each(|r| println!("{:?}", r.get_variant_count()));

    let sum = records
        .iter()
        .fold(0, |acc, val| acc + val.get_variant_count());

    println!("Part 1: {}", sum);
}

// Creating expanded grid
fn part_2(reader: &str) {
    let lines = reader.lines();

    let records = lines.map(|line| Record::from(line)).collect::<Vec<_>>();

    let records2 = records
        .iter()
        .map(|r| {
            let mut nr = Vec::new();
            nr.extend(r.springs.clone());
            nr.extend(r.springs.clone());
            nr.extend(r.springs.clone());
            nr.extend(r.springs.clone());
            nr.extend(r.springs.clone());
            let mut ns = Vec::new();
            ns.extend(r.groups.clone());
            ns.extend(r.groups.clone());
            ns.extend(r.groups.clone());
            ns.extend(r.groups.clone());
            ns.extend(r.groups.clone());
            Record {
                springs: nr,
                groups: ns,
            }
        })
        .collect::<Vec<_>>();

    // records.iter().for_each(|r| println!("{:?}", r));

    // records
    //     .iter()
    //     .for_each(|r| println!("{:?}", r.get_variant_count()));
    println!("Starting");

    let sum = records2.iter().fold(0, |acc, val| {
        println!("Acc: {acc}");
        acc + val.get_variant_count()
    });

    println!("Part 2: {}", sum);
}
