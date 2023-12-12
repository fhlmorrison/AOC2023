use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::sync::Mutex;
use std::time::Instant;

static MEMO: Lazy<Mutex<HashMap<(Vec<char>, Vec<usize>), usize>>> = Lazy::new(Default::default);

fn main() {
    // let reader = read_to_string("./inputs/test12A.txt").unwrap();
    let reader = read_to_string("./inputs/day12.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

#[derive(Debug)]
struct Record {
    springs: Vec<char>,
    groups: Vec<usize>,
}

fn memo_get(input: &(Vec<char>, Vec<usize>)) -> Option<usize> {
    let memo = MEMO.lock().unwrap();
    memo.get(input).map(|v| *v)
}

fn memo_set(key: (Vec<char>, Vec<usize>), value: usize) {
    let mut memo = MEMO.lock().unwrap();
    memo.insert(key, value);
}

fn recurse<'a, 'b>(springs: &'b [char], nums: &'b [usize]) -> usize {
    if springs.is_empty() {
        // End of springs
        if nums.is_empty() {
            return 1;
        }
        // println!("End of springs, but not nums: {:?}", nums);
        return 0;
    }

    if nums.is_empty() {
        if springs.contains(&'#') {
            // println!("End of nums, but not springs: {:?}", springs);
            return 0;
        }
        return 1;
    }

    // Memoization
    if let Some(val) = memo_get(&(springs.to_owned(), nums.to_owned())) {
        // println!("Memoized get {}", val);
        return val;
    }

    let mut res = 0;

    if ['.', '?'].contains(&springs[0]) {
        // println!("Op or Unknown");
        res += recurse(&springs[1..], nums);
    }

    if ['#', '?'].contains(&springs[0]) {
        if nums[0] <= springs.len() // Enough springs are left
            && !springs[..nums[0]].contains(&'.') // Only damaged springs in front
            && (nums[0] == springs.len() || springs[nums[0]] != '#')
        {
            let next: &[char];
            if nums[0] == springs.len() {
                next = &[];
            } else {
                next = &springs[nums[0] + 1..]
            }
            res += recurse(next, &nums[1..]);
        }
    }

    memo_set((springs.to_owned(), nums.to_owned()), res);
    res
}

impl From<&str> for Record {
    fn from(s: &str) -> Self {
        let split = s.split_whitespace().collect::<Vec<&str>>();

        let springs = split[0].chars().collect::<Vec<_>>();

        let groups = split[1]
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Record { springs, groups }
    }
}

impl Record {
    fn get_variant_count(&self) -> usize {
        recurse(&self.springs, &self.groups)
    }
}

// Creating expanded grid
fn part_1(reader: &str) {
    let start = Instant::now();
    let lines = reader.par_lines();

    let records = lines.map(|line| Record::from(line)).collect::<Vec<_>>();

    let sum = records
        .iter()
        .fold(0, |acc, val| acc + val.get_variant_count());

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

// Creating expanded grid
fn part_2(reader: &str) {
    let start = Instant::now();
    let lines = reader.par_lines();

    let unfolded_records = lines
        .map(|line| Record::from(line))
        .map(|r| {
            let mut nr = r.springs.clone();
            for _ in 0..4 {
                nr.push('?');
                nr.extend(r.springs.clone());
            }
            let mut ns = r.groups.clone();
            for _ in 0..4 {
                ns.extend(r.groups.clone());
            }
            Record {
                springs: nr,
                groups: ns,
            }
        })
        .collect::<Vec<_>>();

    let sum = unfolded_records
        .iter()
        .fold(0, |acc, val| acc + val.get_variant_count());

    println!("Part 2: {} in {:?}", sum, start.elapsed());
}
