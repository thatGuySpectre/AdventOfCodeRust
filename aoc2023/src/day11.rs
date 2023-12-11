use shared::{Solution, Answer};
use itertools::Itertools;

pub struct Day11;

impl Solution for Day11 {
    fn part_1(&self, input: &str) -> Answer {
        Answer::from(solve(input, 2))
    }

    fn part_2(&self, input: &str) -> Answer {
        Answer::from(solve(input, 1_000_000))
    }
}

fn solve(input: &str, inflation_arg: usize) -> u64 {
    let grid = input.lines().map(|line| line.chars().map(|x| x == '#').collect::<Vec<bool>>()).collect::<Vec<_>>();

    let mut distances = vec![vec![1; grid[0].len()]; grid.len()];

    let mut empty_col = vec![true; grid[0].len()];
    for (i, line) in grid.iter().enumerate() {
        empty_col = empty_col.iter().enumerate().map(|(i, &l)| l && !line[i]).collect();
        if !line.iter().any(|&x| x) {
            distances[i] = vec![inflation_arg; line.len()];
        }
    }

    for (i, &col) in empty_col.iter().enumerate() {
        if col {
            for j in 0..distances.len() {
                distances[j][i] = inflation_arg;
            }
        }
    }

    let galaxies: Vec<(usize, usize)> = grid.iter().
        enumerate().
        map(|(i, line)| line.iter()
            .enumerate()
            .filter_map(|(j, &x)| if x { Some((i, j)) } else { None })
            .collect::<Vec<_>>()
        ).flatten().collect::<Vec<_>>();

    galaxies.iter().permutations(2).map(|x| {
        let (x1, y1): (usize, usize) = *x[0];
        let (x2, y2): (usize, usize) = *x[1];
        let mut out = 0;
        let xrange = x1.min(x2)..x1.max(x2);
        let yrange = if y1 <= y2 { y1..y2 } else { (y2+1)..(y1+1) };
        for i in xrange { out += distances[i][if x1 <= x2 {y1} else {y2}] }
        for j in yrange { out += distances[x1.min(x2)][j] }
        out as u64
    } ).sum::<u64>() / 2
}
