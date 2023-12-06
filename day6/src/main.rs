use std::fs::read_to_string;

fn main() {
    // let reader = read_to_string("./inputs/test6A.txt").unwrap();
    let reader = read_to_string("./inputs/day6.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

// time = velocity + distance/velocity {t = v + d/v}
// Solve for velocity -> quadratic roots {v = (t +- sqrt(t^2 - 4d)) / 2}
// and the difference between the roots is the number of ways to beat the time

fn part_1(reader: &str) {
    let lines = reader.lines();
    let nums = lines
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let games = std::iter::zip(nums[0].iter(), nums[1].iter());
    let roots = games.map(roots).collect::<Vec<_>>();

    let sum = roots.iter().map(|(a, b)| a - b).fold(1, |acc, x| acc * x);

    // println!("roots: {:?}", roots);

    println!("Part 1: {}", sum);
}

fn part_2(reader: &str) {
    let lines = reader.lines();
    let nums = lines
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .split_whitespace()
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    // println!("nums: {:?}", nums);

    let game = (
        &nums[0].parse::<f64>().unwrap(),
        &nums[1].parse::<f64>().unwrap(),
    );
    let roots = roots2(game);

    let sum = roots.0 - roots.1;

    // println!("roots: {:?}", roots);

    println!("Part 2: {}", sum);
}

fn roots<'a>(game: (&u32, &u32)) -> (u32, u32) {
    let (t, d) = game;

    // Quadratic determinant
    let det = f64::from(t * t - 4 * d);
    // println!("det: {}", det);

    let ft = f64::from(*t);

    let root1 = (ft + det.sqrt()) / 2.;
    let root2 = (ft - det.sqrt()) / 2.;

    let (mut upper, mut lower) = (f64::floor(root1) as u32, f64::floor(root2) as u32);
    // println!("upper: {}, lower: {}", upper, lower);

    // Check bounds
    while upper + d / upper >= *t {
        upper -= 1;
    }
    while lower + d / lower >= *t {
        lower += 1;
    }
    // Make bounds non-inclusize on low end for solution being the difference
    lower -= 1;

    (upper, lower)
}

// Had to make a new function for part 2 because number was too big for u32
fn roots2<'a>(game: (&f64, &f64)) -> (u64, u64) {
    let (t, d) = game;

    // Quadratic determinant
    let det = f64::from(t * t - 4. * d);
    // println!("det: {}", det);

    let ft = f64::from(*t);

    let mut root1 = (ft + det.sqrt()) / 2.;
    let mut root2 = (ft - det.sqrt()) / 2.;

    // println!("upper: {}, lower: {}", upper, lower);

    // Check bounds to account for float precision
    while root1 + d / root1 >= *t {
        root1 -= 1.;
    }
    while root2 + d / root2 >= *t {
        root2 += 1.;
    }
    // Make bounds non-inclusize on low end for solution being the difference
    root2 -= 1.;
    let (upper, lower) = (f64::floor(root1) as u64, f64::floor(root2) as u64);

    (upper, lower)
}
