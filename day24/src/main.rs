use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    // let reader = read_to_string("./inputs/test24A.txt").unwrap();
    let reader = read_to_string("./inputs/day24.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let start = Instant::now();
    let lines = reader.lines();

    let hailstones = lines.map(|line| Hailstone::from(line)).collect::<Vec<_>>();

    // let bounds = (7f64, 27f64);
    let bounds = (200000000000000f64, 400000000000000f64);

    let mut sum = 0;
    for (i, first_hailstone) in hailstones.iter().enumerate() {
        for second_hailstone in hailstones[i + 1..].iter() {
            if first_hailstone.position == second_hailstone.position {
                continue;
            }
            if let Some((x, y)) = first_hailstone.intersect_xy(second_hailstone) {
                if x >= bounds.0 && x <= bounds.1 && y >= bounds.0 && y <= bounds.1 {
                    sum += 1;
                }
            }
        }
    }

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

// I'm not familiar with linalg solvers for rust, so I caved and used Z3PY
// I might revisit this later and try to implement a solution in rust
fn part_2(reader: &str) {
    println!("Part 2: Use python script (day24/part2.py)")
}

struct Hailstone {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl From<&str> for Hailstone {
    fn from(s: &str) -> Self {
        let mut split = s.split('@');
        let mut positions = split.next().unwrap().trim().split(", ");
        let mut velocities = split
            .next()
            .unwrap()
            .trim()
            .split(", ")
            .map(|s| s.trim())
            .collect::<Vec<_>>();
        // println!("{:?}", velocities);
        let x = positions.next().unwrap().parse::<f64>().unwrap();
        let y = positions.next().unwrap().parse::<f64>().unwrap();
        let z = positions.next().unwrap().parse::<f64>().unwrap();
        let vx = velocities[0].parse::<f64>().unwrap();
        let vy = velocities[1].parse::<f64>().unwrap();
        let vz = velocities[2].parse::<f64>().unwrap();
        Hailstone {
            position: (x, y, z),
            velocity: (vx, vy, vz),
        }
    }
}

impl Hailstone {
    fn intersect_xy(&self, other: &Hailstone) -> Option<(f64, f64)> {
        let (x1, y1, _) = self.position;
        let (x2, y2, _) = other.position;
        let (vx1, vy1, _) = self.velocity;
        let (vx2, vy2, _) = other.velocity;

        let det = vx1 * vy2 - vy1 * vx2;

        if det.abs() < f64::EPSILON {
            // Lines are parallel or coincident
            return None;
        }

        let t = ((x2 - x1) * vy2 - (y2 - y1) * vx2) / det;
        let s = ((x2 - x1) * vy1 - (y2 - y1) * vx1) / det;

        // Check if the intersection is in the future for both hailstones (t > 0 and s > 0)
        if t > 0.0 && s > 0.0 {
            let x = x1 + vx1 * t;
            let y = y1 + vy1 * t;
            Some((x, y))
        } else {
            None
        }
    }

    fn cross_product(&self, other: &Hailstone) -> f64 {
        let (x1, y1, _) = self.position;
        let (x2, y2, _) = other.position;
        let (vx1, vy1, _) = self.velocity;
        let (vx2, vy2, _) = other.velocity;

        (x2 - x1) * vy1 - (y2 - y1) * vx1
    }
}

fn dot_product(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn subtract(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn find_intersection(h1: Hailstone, h2: Hailstone) -> Option<(f64, f64, f64)> {
    let w = subtract(h1.position, h2.position);
    let a = dot_product(h1.velocity, h1.velocity);
    let b = dot_product(h1.velocity, h2.velocity);
    let c = dot_product(h2.velocity, h2.velocity);
    let d = dot_product(h1.velocity, w);
    let e = dot_product(h2.velocity, w);

    let denominator = a * c - b * b;
    if denominator.abs() < f64::EPSILON {
        return None; // Lines are parallel or coincident
    }

    let t = (b * e - c * d) / denominator;
    let s = (a * e - b * d) / denominator;

    let point_on_h1 = (
        h1.position.0 + t * h1.velocity.0,
        h1.position.1 + t * h1.velocity.1,
        h1.position.2 + t * h1.velocity.2,
    );
    let point_on_h2 = (
        h2.position.0 + s * h2.velocity.0,
        h2.position.1 + s * h2.velocity.1,
        h2.position.2 + s * h2.velocity.2,
    );

    if subtract(point_on_h1, point_on_h2) == (0.0, 0.0, 0.0) {
        Some(point_on_h1) // Intersection point
    } else {
        None // Lines do not intersect
    }
}
