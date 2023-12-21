use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    // let reader = read_to_string("./inputs/test21A.txt").unwrap();
    let reader = read_to_string("./inputs/day21.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let start = Instant::now();
    let matrix = reader
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_tile = find_start(&matrix);

    let iterations = 64;

    let sum = walk_garden(&matrix, start_tile, iterations);

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

// Does not work on test input due to assumptions made based on real input
fn part_2(reader: &str) {
    let start = Instant::now();
    let mut matrix = reader
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_tile = find_start(&matrix);

    matrix[start_tile.1][start_tile.0] = '.';

    let total_iterations = 26501365;

    // Assume square matrix of odd length & start in the middle
    let size = matrix.len();

    let grid_width = total_iterations / size - 1;

    let odd_grids = (grid_width / 2 * 2 + 1) * (grid_width / 2 * 2 + 1);
    let even_grids = ((grid_width + 1) / 2 * 2) * ((grid_width + 1) / 2 * 2);

    let odd_sum = walk_garden(&matrix, start_tile, size * 2 + 1);
    let even_sum = walk_garden(&matrix, start_tile, size * 2);

    let corner_iters = size - 1;

    let n_sum = walk_garden(&matrix, (start_tile.0, size - 1), corner_iters);
    let w_sum = walk_garden(&matrix, (size - 1, start_tile.1), corner_iters);
    let e_sum = walk_garden(&matrix, (0, start_tile.1), corner_iters);
    let s_sum = walk_garden(&matrix, (start_tile.0, 0), corner_iters);

    let small_edge_iters = corner_iters - (size / 2 + 1);
    let small_edges = grid_width + 1;

    let nw_small_sum = walk_garden(&matrix, (size - 1, size - 1), small_edge_iters);
    let ne_small_sum = walk_garden(&matrix, (0, size - 1), small_edge_iters);
    let sw_small_sum = walk_garden(&matrix, (size - 1, 0), small_edge_iters);
    let se_small_sum = walk_garden(&matrix, (0, 0), small_edge_iters);

    let large_edge_iters = (corner_iters + size) - (size / 2 + 1);
    let large_edges = grid_width;

    let nw_large_sum = walk_garden(&matrix, (size - 1, size - 1), large_edge_iters);
    let ne_large_sum = walk_garden(&matrix, (0, size - 1), large_edge_iters);
    let sw_large_sum = walk_garden(&matrix, (size - 1, 0), large_edge_iters);
    let se_large_sum = walk_garden(&matrix, (0, 0), large_edge_iters);

    let mut sum = 0;

    sum += odd_sum * odd_grids;
    sum += even_sum * even_grids;
    sum += n_sum;
    sum += w_sum;
    sum += e_sum;
    sum += s_sum;
    sum += nw_small_sum * small_edges;
    sum += ne_small_sum * small_edges;
    sum += sw_small_sum * small_edges;
    sum += se_small_sum * small_edges;
    sum += nw_large_sum * large_edges;
    sum += ne_large_sum * large_edges;
    sum += sw_large_sum * large_edges;
    sum += se_large_sum * large_edges;

    println!("Part 2: {} in {:?}", sum, start.elapsed());
}

fn find_start(matrix: &Vec<Vec<char>>) -> (usize, usize) {
    matrix
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, c)| (x, y, c))
                .collect::<Vec<_>>()
        })
        .flatten()
        .find(|(_, _, c)| *c == &'S')
        .map(|(x, y, _)| (x, y))
        .unwrap()
}

fn walk_garden(matrix: &Vec<Vec<char>>, start_tile: (usize, usize), iterations: usize) -> usize {
    let mut reachable_matrix = matrix.clone();
    reachable_matrix[start_tile.1][start_tile.0] = 'O';

    for _ in 0..iterations {
        let mut new_reachable_matrix = reachable_matrix.clone();
        for y in 0..reachable_matrix.len() {
            for x in 0..reachable_matrix[y].len() {
                if reachable_matrix[y][x] == 'O' {
                    if y > 0 && reachable_matrix[y - 1][x] == '.' {
                        new_reachable_matrix[y - 1][x] = 'O';
                    }
                    if y < reachable_matrix.len() - 1 && reachable_matrix[y + 1][x] == '.' {
                        new_reachable_matrix[y + 1][x] = 'O';
                    }
                    if x > 0 && reachable_matrix[y][x - 1] == '.' {
                        new_reachable_matrix[y][x - 1] = 'O';
                    }
                    if x < reachable_matrix[y].len() - 1 && reachable_matrix[y][x + 1] == '.' {
                        new_reachable_matrix[y][x + 1] = 'O';
                    }
                    new_reachable_matrix[y][x] = '.';
                }
            }
        }
        reachable_matrix = new_reachable_matrix;
    }

    // reachable_matrix.iter().for_each(|row| {
    //     row.iter().for_each(|c| {
    //         print!("{}", c);
    //     });
    //     println!();
    // });

    let mut sum = reachable_matrix
        .iter()
        .map(|row| row.iter().filter(|c| **c == 'O').count())
        .sum::<usize>();
    sum
}

// Iter 0
//  ^
// <.>
//  V
//
// Iter 1
//   ^
//  /,\
// <,.,>
//  \,/
//   v

// at each iteration full tiles increases by 4 * number of iteration
// grid_width = number of iterations / size of tile -1
// 2 types of full tile, odd and even
// odd grids = grid_width rounded down to even + 1
// even grids = grid_width rounded up to even

// one type of corner, only one instance for each direction

// 2 types of edge for each direction, small and large
// small edge = grid_width + 1
// large edge = grid_width

// 1, 5, 8, 13, 25, 41

// each corner is always 1, so 4 total

// each edge is number of iteration
