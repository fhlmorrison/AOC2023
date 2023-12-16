use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    // let reader = read_to_string("./inputs/test15A.txt").unwrap();
    let reader = read_to_string("./inputs/day15.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let start = Instant::now();
    let steps = reader.split(',');
    let sum = steps.fold(0, |acc, step| acc + hashing_algorithm(step));

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

fn part_2(reader: &str) {
    let start = Instant::now();
    let steps = reader.split(',');

    let mut boxes: [Vec<Option<Lens>>; 256] = core::array::from_fn(|_| Vec::new());

    for step in steps {
        if step.ends_with('-') {
            let label = step[..step.len() - 1].to_string();
            let hash = hashing_algorithm(&label);

            if let Some(slot) = boxes[hash]
                .iter()
                .position(|x| x.as_ref().is_some_and(|x| x.label == label))
            {
                boxes[hash][slot] = None;
            }
            continue;
        }
        let mut split = step.split('=');
        let label = split.next().unwrap().to_string();
        let hash = hashing_algorithm(&label);
        let num = split.next().unwrap().parse::<usize>().unwrap();

        if let Some(slot) = boxes[hash]
            .iter()
            .position(|x| x.as_ref().is_some_and(|x| x.label == label))
        {
            let lens = Lens { label, num };
            boxes[hash][slot] = Some(lens);
        } else {
            let lens = Lens { label, num };
            boxes[hash].push(Some(lens));
        }
    }

    // Clean up removed lenses
    for i in 0..boxes.len() {
        boxes[i].retain(|x| x.is_some());
    }

    let sum = boxes.iter().enumerate().fold(0, |acc, (i, lenses)| {
        acc + lenses
            .iter()
            .enumerate()
            .map(|(slot, x)| {
                let lens = x.as_ref().unwrap();
                lens.num * (i + 1) * (slot + 1) // focal power
            })
            .sum::<usize>()
    });

    println!("Part 2: {} in {:?}", sum, start.elapsed());
}

fn hashing_algorithm(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

#[derive(Default, Debug)]
struct Lens {
    label: String,
    num: usize,
}
