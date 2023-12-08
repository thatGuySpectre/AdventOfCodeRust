use std::iter::zip;
use shared::{Solution, Answer};

pub struct Day06;

impl Solution for Day06 {
    fn part_1(&self, input: &str) -> Answer {
        let data: Vec<&str> = input.split("\n").collect();
        let times: Vec<u64> = data[0].split_whitespace().skip(1).map(|x| x.parse::<u64>().unwrap()).collect();
        let dists: Vec<u64> = data[1].split_whitespace().skip(1).map(|x| x.parse::<u64>().unwrap()).collect();

        let mut out: u64 = 1;

        for (t, d) in zip(times, dists) {
            let mut win: u64 = 0;
            for time in 1..t {
                if time * (t-time) > d { win +=1 }
            }
            out *= win
        }

        Answer::from(out)
    }

    fn part_2(&self, input: &str) -> Answer {
        let data: Vec<&str> = input.split("\n").collect();
        let time: u64 = data[0].split_whitespace().skip(1).collect::<Vec<&str>>().join("").parse::<u64>().unwrap();
        let dist: u64 = data[1].split_whitespace().skip(1).collect::<Vec<&str>>().join("").parse::<u64>().unwrap();

        let mut win: u64 = 0;

        for t in 1..time {
            if t * (time - t) > dist { win += 1 }
        }

        Answer::from(win)
    }
}

