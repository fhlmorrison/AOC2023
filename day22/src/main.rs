use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::fs::read_to_string;
use std::time::Instant;

// Stuff runs slow cause Rust uses timing attack safe hashing which is slow
// and I am doing a lot of HashSet and HashMap operations

// Assumptions: z1 < z2, x1 < x2, y1 < y2

fn main() {
    // let reader = read_to_string("./inputs/test22A.txt").unwrap();
    let reader = read_to_string("./inputs/day22.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let start = Instant::now();
    let lines = reader.lines();

    let bricks = lines.map(|s| Brick::from(s)).collect::<Vec<_>>();
    let settled = settle_bricks(&bricks);

    // for each brick, find all bricks directly below it
    // count all bricks that are the single support of another brick

    let support_map = create_support_map(&settled);
    let single_supports = support_map
        .values()
        .filter(|supports| supports.len() == 1)
        .map(|supports| supports.iter().next().unwrap())
        .collect::<HashSet<_>>();

    let sum = settled.len() - single_supports.len();
    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

fn part_2(reader: &str) {
    let start = Instant::now();
    let lines = reader.lines();

    let bricks = lines.map(|s| Brick::from(s)).collect::<Vec<_>>();
    let settled = settle_bricks(&bricks);

    let support_map = create_support_map(&settled);

    let mut sum = 0;
    for brick in settled.iter() {
        sum += chain_reaction(brick, &support_map);
    }

    println!("Part 2: {} in {:?}", sum, start.elapsed());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    x1: isize,
    y1: isize,
    z1: isize,
    x2: isize,
    y2: isize,
    z2: isize,
}

impl From<&str> for Brick {
    fn from(s: &str) -> Self {
        let mut sides = s.split('~');

        let mut first = sides.next().unwrap().split(',');
        let mut second = sides.next().unwrap().split(',');
        let x1 = first.next().unwrap().parse::<isize>().unwrap();
        let y1 = first.next().unwrap().parse::<isize>().unwrap();
        let z1 = first.next().unwrap().parse::<isize>().unwrap();
        let x2 = second.next().unwrap().parse::<isize>().unwrap();
        let y2 = second.next().unwrap().parse::<isize>().unwrap();
        let z2 = second.next().unwrap().parse::<isize>().unwrap();
        Brick {
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
        }
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}~{},{},{}",
            self.x1, self.y1, self.z1, self.x2, self.y2, self.z2
        )
    }
}

// This is slow
fn settle_bricks(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut sorted = bricks.clone();
    sorted.sort_by_key(|brick| brick.z1.min(brick.z2));

    let mut settled = Vec::<Brick>::new();
    sorted.iter().for_each(|brick| {
        // lower brick until it hits another or the ground
        let min_z = brick.z1.min(brick.z2);
        let mut min_possible = min_z;

        'outer: loop {
            if min_possible == 1 {
                // Brick cannot go lower
                break;
            }
            for other in settled.iter() {
                // Check if any part of the other brick is below the brick
                for i in other.x1..=other.x2 {
                    for j in other.y1..=other.y2 {
                        if (brick.x1..=brick.x2).contains(&i)
                            && (brick.y1..=brick.y2).contains(&j)
                            && (other.z2 == min_possible - 1)
                        {
                            // Brick cannot go lower
                            break 'outer;
                        }
                    }
                }
            }
            min_possible -= 1;
        }
        settled.push(Brick {
            x1: brick.x1,
            y1: brick.y1,
            z1: min_possible,
            x2: brick.x2,
            y2: brick.y2,
            z2: min_possible + brick.z2 - brick.z1,
        });
    });
    settled
}

// Create a map of brick to all bricks that are directly below (supporting) it
fn create_support_map<'a>(
    settled_bricks: &'a Vec<Brick>,
) -> HashMap<&'a Brick, HashSet<&'a Brick>> {
    HashMap::<&Brick, HashSet<&Brick>>::from_iter(settled_bricks.iter().map(|brick| {
        let mut supports = HashSet::<&Brick>::new();
        settled_bricks.iter().for_each(|other| {
            // Check if any part of the other brick is directly below (supports) the brick
            for i in other.x1..=other.x2 {
                for j in other.y1..=other.y2 {
                    if (brick.x1..=brick.x2).contains(&i)
                        && (brick.y1..=brick.y2).contains(&j)
                        && (other.z2 == brick.z1 - 1)
                    {
                        supports.insert(other);
                    }
                }
            }
        });
        (brick, supports)
    }))
}

// Optimized after submitting
fn chain_reaction(brick: &Brick, support_map: &HashMap<&Brick, HashSet<&Brick>>) -> usize {
    let mut disintegrated_bricks = HashSet::<&Brick>::new();
    disintegrated_bricks.insert(brick);
    let mut local_sum = 0;

    // println!("Brick: {}", brick);

    let mut change_occured = true;
    // This is the slowest part
    while change_occured {
        change_occured = false;
        for other in support_map.keys() {
            if disintegrated_bricks.contains(other) {
                continue;
            }
            if support_map[other].is_subset(&disintegrated_bricks) && support_map[other].len() > 0 {
                disintegrated_bricks.insert(other);
                local_sum += 1;
                change_occured = true;
            }
        }
    }
    local_sum
}

// What I used for getting my submission
fn old_chain_reaction(brick: &Brick, support_map: &HashMap<&Brick, HashSet<&Brick>>) -> usize {
    let mut local_support_map = support_map.clone();
    let mut queue = VecDeque::<&Brick>::new();
    let mut disintegrated_bricks = HashSet::<&Brick>::new();
    queue.push_back(brick);
    disintegrated_bricks.insert(brick);
    let mut local_sum = 0;

    // This is the slowest part
    while let Some(current_brick) = queue.pop_front() {
        // remove brick from support list if other is supported by brick
        // if support list is empty, brick is unsupported (add to queue and sum += 1)
        for other in support_map.keys() {
            if disintegrated_bricks.contains(other) {
                continue;
            }
            if local_support_map[other].contains(current_brick) {
                local_support_map
                    .get_mut(other)
                    .unwrap()
                    .remove(current_brick);
                if local_support_map[other].is_empty() {
                    queue.push_back(other);
                    disintegrated_bricks.insert(other);
                    local_sum += 1;
                }
            }
        }
    }
    local_sum
}

fn old_part_1(reader: &str) {
    let start = Instant::now();
    let lines = reader.lines();

    let bricks = lines.map(|s| Brick::from(s)).collect::<Vec<_>>();
    let settled = settle_bricks(&bricks);

    // for each brick, find all bricks directly below it
    // record all bricks that are the only brick below another brick

    let mut single_supports = HashSet::<&Brick>::new();

    settled.iter().for_each(|brick| {
        let mut supports = HashSet::<&Brick>::new();
        settled.iter().for_each(|other| {
            // Check if any part of the other brick is directly below the brick
            for i in other.x1..=other.x2 {
                for j in other.y1..=other.y2 {
                    if (brick.x1..=brick.x2).contains(&i)
                        && (brick.y1..=brick.y2).contains(&j)
                        && (other.z2 == brick.z1 - 1)
                    {
                        supports.insert(other);
                    }
                }
            }
        });
        if supports.len() == 1 {
            single_supports.insert(supports.iter().next().unwrap());
        }
    });

    let sum = settled.len() - single_supports.len();
    println!("Part 1: {} in {:?}", sum, start.elapsed());
}
