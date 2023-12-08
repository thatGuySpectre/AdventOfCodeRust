use shared::{Answer, Solution};
use regex::{Regex};

pub struct Day02;

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
            let mut max_red: u64 = 0;
            let mut max_blue: u64 = 0;
            let mut max_green: u64 = 0;

            for m in re.captures_iter(&game) {
                match m.extract::<2>().1 {
                    [n, "red"] => max_red = max_red.max(n.parse::<u64>().unwrap()),
                    [n, "blue"] => max_blue = max_blue.max(n.parse::<u64>().unwrap()),
                    [n, "green"] => max_green = max_green.max(n.parse::<u64>().unwrap()),
                    other => println!("{:?}", other),
                }
            }

            if max_red <= POSSIBLE_RED && max_blue <= POSSIBLE_BLUE && max_green <= POSSIBLE_GREEN {
                sum += (i as u64) + 1;
            }
        }

        Answer::from(sum)
    }

    fn part_2(&self, input: &str) -> Answer {
        let games: Vec<&str> = input.split('\n')
            .collect();

        let re = Regex::new(r"(\d+) (\w+)").unwrap();

        let mut sum: u64 = 0;

        for (_, game) in games.iter().enumerate() {
            let mut min_red: u64 = 0;
            let mut min_blue: u64 = 0;
            let mut min_green: u64 = 0;

            for m in re.captures_iter(&game) {
                match m.extract::<2>().1 {
                    [n, "red"] => min_red = min_red.max(n.parse::<u64>().unwrap()),
                    [n, "blue"] => min_blue = min_blue.max(n.parse::<u64>().unwrap()),
                    [n, "green"] => min_green = min_green.max(n.parse::<u64>().unwrap()),
                    other => println!("{:?}", other),
                }
            }

            sum += min_red * min_blue * min_green;
        }

        Answer::from(sum)
    }
}