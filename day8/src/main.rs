use std::{collections::HashMap, fs::read_to_string};

fn main() {
    // let reader = read_to_string("./inputs/test8A.txt").unwrap();
    let reader = read_to_string("./inputs/day8.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    fn from_str(s: &str) -> Node {
        let mut split = s.split(" = ");
        let name = split.next().unwrap();
        let mut children = split
            .next()
            .unwrap()
            .trim_matches(&['(', ')'] as &[_])
            .split(", ");
        let left = children.next().unwrap();
        let right = children.next().unwrap();

        Node { name, left, right }
    }
}

fn part_1(reader: &str) {
    let mut lines = reader.lines();

    let instructions = lines.next().unwrap();
    let _space = lines.next().unwrap();
    let nodes = lines.map(|s| Node::from_str(s)).collect::<Vec<_>>();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    nodes.iter().for_each(|n| {
        map.insert(n.name, (n.left, n.right));
    });

    let mut sum = 0;
    let mut curr: &str = "AAA";
    let mut i = 0;
    let len = instructions.len();
    while curr != "ZZZ" {
        let instruction = instructions.chars().nth(i).unwrap();

        curr = match instruction {
            'L' => map.get(curr).unwrap().0,
            'R' => map.get(curr).unwrap().1,
            _ => panic!("Invalid instruction"),
        };
        sum += 1;
        i = (i + 1) % len;
    }

    println!("Part 2: {}", sum);
}

fn part_2(reader: &str) {
    let mut lines = reader.lines();

    let instructions = lines.next().unwrap();
    let _space = lines.next().unwrap();
    let nodes = lines.map(|s| Node::from_str(s)).collect::<Vec<_>>();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    nodes.iter().for_each(|n| {
        map.insert(n.name, (n.left, n.right));
    });

    let starting_nodes = nodes
        .iter()
        .filter(|n| n.name.ends_with('A'))
        .map(|n| n.name)
        .collect::<Vec<_>>();

    let sums = starting_nodes.iter().map(|node| {
        let mut sum: usize = 0;
        let mut i = 0;
        let len = instructions.len();
        let mut curr = *node;
        while !curr.ends_with('Z') {
            let instruction = instructions.chars().nth(i).unwrap();

            curr = match instruction {
                'L' => map.get(curr).unwrap().0,
                'R' => map.get(curr).unwrap().1,
                _ => panic!("Invalid instruction"),
            };
            sum += 1;
            i = (i + 1) % len;
        }
        sum
    });

    let sum = sums.fold(1, |acc, x| lcm(acc, x));

    println!("Part 2: {}", sum);
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
