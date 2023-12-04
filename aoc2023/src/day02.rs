use std::any::Any;
use std::collections::HashMap;
use shared::{Answer, Solution};
use regex::{Regex, Captures};

pub struct Day02;

const POSSIBLE: HashMap<&str, u64> = HashMap::from([
    ("red", 12),
    ("green", 13),
    ("blue", 14)
]);

const POSSIBLE_RED : u64 = 12;
const POSSIBLE_BLUE : u64 = 14;
const POSSIBLE_GREEN : u64 = 13;



impl Solution for Day02 {
    fn part_1(&self, input: &str) -> Answer {
        let games: Vec<&str> = input.split("\n")
            .collect();

        let re = Regex::new(r"(\d+) (\w+)").unwrap();

        let mut sum: u64 = 0;

        for (i, game) in games.iter().enumerate() {
            let mut max_red : u64 = 0;
            let mut max_blue : u64 = 0;
            let mut max_green : u64 = 0;

            for m in re.captures_iter(&game) {
                match m.extract() {
                    Some("red") => max_red = max_red.max(m[1].parse::<u64>().unwrap()),
                    Some("blue") => max_blue = max_blue.max(m[1].parse::<u64>().unwrap()),
                    Some("green") => max_green = max_green.max(m[1].parse::<u64>().unwrap()),
                    _ => (),
                }
            }

            if max_red <= POSSIBLE_RED && max_blue <= POSSIBLE_BLUE && max_green <= POSSIBLE_GREEN {
                sum += i + 1;
            }
        }

        sum
    }

    fn part_2(&self, input: &str) -> Answer {
        Answer::Unimplemented
    }
}