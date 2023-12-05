use std::fs::read_to_string;

fn main() {
    let reader = read_to_string("./inputs/day5.txt").unwrap();
    // let reader = read_to_string("./inputs/test5A.txt").unwrap();

    part_1(&reader);
    // part_2(&reader);
}

#[derive(Debug, Clone)]
struct Category(Vec<Range>);
impl Category {
    pub fn map_value(&self, value: u64) -> u64 {
        for i in &self.0 {
            match i.cmp_num(value) {
                std::cmp::Ordering::Greater => continue,
                std::cmp::Ordering::Equal => return i.map_value(value),
                std::cmp::Ordering::Less => return value,
            }
        }
        value
    }
}
#[derive(Debug, Clone)]
struct Range {
    pub source: u64,
    pub destination: u64,
    pub length: u64,
}

impl Range {
    pub fn map_value(&self, value: u64) -> u64 {
        if value >= self.source && value < self.source + self.length {
            let diff = value - self.source;
            self.destination + diff
        } else {
            0
        }
    }

    pub fn cmp_num(&self, num: u64) -> std::cmp::Ordering {
        if num >= self.source && num < self.source + self.length {
            std::cmp::Ordering::Equal
        } else if num < self.source {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

fn parse_file(reader: &str) -> (Vec<u64>, Vec<Category>) {
    let categories = reader.split("\r\n\r\n");

    let cat_arrs: Vec<_> = categories
        .map(|s| s.split(':').collect::<Vec<_>>())
        .collect();

    let mut cat_nums = cat_arrs.iter().map(|s| {
        s[1].trim()
            .lines()
            .map(|s| {
                s.split_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    });

    let seeds = cat_nums.next().unwrap().first().unwrap().clone();
    let mut other_cats = cat_nums.collect::<Vec<_>>();

    let mut cat_ranges = other_cats
        .iter_mut()
        .map(|cat| Category {
            0: cat
                .iter_mut()
                .map(|line| Range {
                    destination: line[0],
                    source: line[1],
                    length: line[2],
                })
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();

    (seeds, cat_ranges)
}

fn part_1(reader: &str) {
    // let lines: Vec<_> = reader.lines().collect();

    let (seeds, mut cat_ranges) = parse_file(reader);
    // let category_maps: Vec<_> = cat_nums.collect();
    cat_ranges
        .iter_mut()
        .for_each(|s| s.0.sort_by_key(|r| r.source));

    let locations = seeds.iter().map(|seed| {
        // println!("Completed seed: {:?}", seed);
        all_map_value(seed.clone(), &cat_ranges)
    });

    let sum = locations.min().unwrap();

    println!("Part 1: {}", sum);
}

fn part_2(reader: &str) {
    let (seeds, mut cat_ranges) = parse_file(reader);
    // let category_maps: Vec<_> = cat_nums.collect();
    cat_ranges
        .iter_mut()
        .for_each(|s| s.0.sort_by_key(|r| r.source));

    let seed_ranges = seeds.chunks(2).collect::<Vec<_>>();

    let mut join_handles: Vec<std::thread::JoinHandle<_>> = Vec::new();
    for seed_range in seed_ranges {
        let seed = seed_range.to_owned().to_owned();
        let cat_ranges2 = cat_ranges.to_owned();
        join_handles.push(std::thread::spawn(move || {
            println!("Starting seed: {:?}", seed);
            let res = (seed[0]..=(seed[1] + seed[0]))
                .reduce(|acc, val| std::cmp::min(all_map_value(val.clone(), &cat_ranges2), acc))
                .unwrap();
            println!("Completed seed: {:?}, result: {res}", seed);
            res
        }));
    }

    let locations = join_handles.into_iter().map(|i| i.join().unwrap());

    let sum = locations.min().unwrap();

    println!("Part 1: {}", sum);
}

fn all_map_value(value: u64, cats: &Vec<Category>) -> u64 {
    cats.iter().fold(value, |acc, cat| cat.map_value(acc))
}
