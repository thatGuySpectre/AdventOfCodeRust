use std::iter;
use std::iter::zip;
use itertools::Itertools;
use shared::{Solution, Answer};

pub struct Day12;

impl Solution for Day12 {
    fn part_1(&self, input: &str) -> Answer {
        let lines = input.lines()
            .map(|x| x.split_once(' ').unwrap().0.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let groups: Vec<Vec<u64>> = input.lines()
            .map(|x| x.split_once(' ').unwrap().1.split(',').map(|y| y.parse::<u64>().unwrap()).collect())
            .collect();

        let mut out = 0;

        for (line, group) in zip(lines, groups) {
            out += solve(&line, &group)
        }

        Answer::from(out)
    }

    fn part_2(&self, input: &str) -> Answer {
        let lines = input.lines()
            .map(|x| iter::repeat(x.split_once(' ').unwrap().0).take(5).join("?").chars().collect())
            .collect::<Vec<Vec<char>>>();
        let groups: Vec<Vec<u64>> = input.lines()
            .map(|x| iter::repeat(
                x.split_once(' ')
                .unwrap().1
                .split(',')
                .map(|a| a.parse::<u64>().unwrap())
            ).take(5)
                .flatten()
                .collect()
            ).collect();

        let mut out = 0;

        for (line, group) in zip(lines, groups) {
            out += solve(&line, &group)
        }

        Answer::from(out)
    }
}

fn solve(springs: &Vec<char>, groups: &Vec<u64>) -> u64 {
    let n = springs.len();
    let m = groups.len();

    let mut possibilities: Vec<Vec<u64>> = vec![vec![0; m+1]; n+1];

    possibilities[0][0] = 1;

    for i in 0..n {
        for j in 0..=m {
            if springs[i] != '#' {
                possibilities[i+1][j] += possibilities[i][j];
            }
            if j < m
                && (i + groups[j] as usize) <= n
                && !springs[i..(i + groups[j] as usize)].contains(&'.')
                && (i + groups[j] as usize >= n || springs[i + groups[j] as usize ] != '#') {
                possibilities[(i + groups[j] as usize + 1).min(n)][j+1] += possibilities[i][j]
            }
        }
    }

    possibilities[n][m]
}
