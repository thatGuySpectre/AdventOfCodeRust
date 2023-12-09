use std::iter::zip;
use shared::{Solution, Answer};

pub struct Day09;

impl Solution for Day09 {
    fn part_1(&self, input: &str) -> Answer {
        let lines = input.split("\n")
            .map(|l| l.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
            ).collect::<Vec<Vec<i64>>>();

        Answer::from(
            lines.iter().map(solve).sum::<i64>()
        )
    }

    fn part_2(&self, input: &str) -> Answer {
        let lines = input.split("\n")
            .map(|l| l.split_whitespace().map(|x| x.parse::<i64>().unwrap()).rev().collect()
            ).collect::<Vec<Vec<i64>>>();

        Answer::from(
            lines.iter().map(solve).sum::<i64>()
        )
    }
}

fn solve(seq: &Vec<i64>) -> i64 {
    if seq.iter().all(|x| *x==0) {
        return 0
    }

    let diffs = zip(seq, seq.iter().skip(1))
        .map(|(a, b)| b - a )
        .collect();

    return seq.last().unwrap() + solve(&diffs)
}
