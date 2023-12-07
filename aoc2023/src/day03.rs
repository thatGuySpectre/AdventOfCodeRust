use shared::{Answer, Solution};
use regex::Regex;

pub struct Day03;

impl Solution for Day03 {
    fn part_1(&self, input: &str) -> Answer {
        let lines: Vec<&str> = input.split("\n").collect();

        let pattern = Regex::new(
            // r"[^\d.^]?(\d+)[\d.]|[^\d.](\d+)[\d.$]?"
            r"(\d+)"
        ).unwrap();

        let mut sum: u64 = 0;

        for i in 0..lines.len() {
            for m in pattern.find_iter(lines[i]) {
                match lines.get(i-1) {
                    _ => (),
                }
            }
        }

        Answer::Unimplemented
    }

    fn part_2(&self, input: &str) -> Answer {
        todo!()
    }
}
