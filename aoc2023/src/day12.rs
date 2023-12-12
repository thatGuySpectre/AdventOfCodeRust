use std::fmt::format;
use std::iter::zip;
use std::net::Shutdown::Read;
use itertools::Itertools;
use shared::{Solution, Answer};
use regex::Regex;
use shared::Answer::String;

pub struct Day12;

impl Solution for Day12 {
    fn part_1(&self, input: &str) -> Answer {
        let lines = input.lines()
            .map(|x| x.split_once(' ').unwrap().0)
            .collect::<Vec<&str>>();
        let groups: Vec<Vec<i32>> = input.lines()
            .map(|x| x.split_once(' ').unwrap().1.split(',').map(|y| y.parse::<i32>().unwrap()).collect())
            .collect();

        let mut out = 0;

        for (line, group) in zip(lines, groups) {
            for pattern in generate_patterns(line.len() as i32, &group) {
                if pattern.is_match(line) {
                    out += 1;
                }
            }
            println!("{out}");
        }

        Answer::from(out)
    }

    fn part_2(&self, input: &str) -> Answer {
        Answer::Unimplemented
    }
}


fn generate_patterns(len_of_str: i32, groups: &Vec<i32>) -> Vec<Regex> {
    let uncertain: i32 = len_of_str - groups.iter().sum::<i32>() - groups.len() as i32 + 1;

    let mut patterns: Vec<Regex> = vec![];
    let possible_gaps = get_gaps(uncertain, groups.len() as i32 + 1);

    for gaps in possible_gaps {
        let mut pattern = format!("^[.?]{{{}}}[#?]{{{}}}", gaps.first().unwrap(), groups.first().unwrap());
        for i in 1..groups.len() {
            pattern.push_str(format!(r"[.?]{{{}}}[#?]{{{}}}", gaps[i] + 1, groups[i]).as_str())
        }
        pattern.push_str(format!("[.?]{{{}}}$", gaps.last().unwrap()).as_str());
        patterns.push(Regex::new(pattern.as_str()).unwrap())
    }

    patterns
}

fn get_gaps(total: i32, parts: i32) -> Vec<Vec<i32>> {
    if total == 0 { return vec![vec![0; parts as usize]] }
    else if parts == 1 { return vec![vec![total]] }

    let mut out: Vec<Vec<i32>> = vec![];

    for i in 0..=total {
        for mut sub in get_gaps(total - i, parts - 1) {
            out.push({ sub.push(i); sub });
        }
    }

    out
}
