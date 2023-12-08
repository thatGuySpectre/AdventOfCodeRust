use shared::{Answer, Solution};
use regex::Regex;
use substring::Substring;

pub struct Day03;

impl Solution for Day03 {
    fn part_1(&self, input: &str) -> Answer {
        let lines: Vec<&str> = input.split('\n').collect();

        let mut out: u64 = 0;

        let re = Regex::new(r"\d+").unwrap();
        let special = Regex::new(r"[^\d.]").unwrap();

        for (i, line) in lines.iter().enumerate() {
            for capture in re.find_iter(line) {
                let prev = if i > 0 { lines.get(i - 1) } else { None };
                let curr = lines.get(i);
                let next = lines.get(i+1);

                if match prev {
                    Some(l) => {
                        let part = l.substring(1.max(capture.start()) - 1, capture.end() + 1);
                        special.is_match(part)
                    },
                    None => false,
                } | match curr {
                    Some(l) => {
                        let part = l.substring(1.max(capture.start()) - 1, capture.end() + 1);
                        special.is_match(part)
                    },
                    None => false,
                } | match next {
                    Some(l) => {
                        let part = l.substring(1.max(capture.start()) - 1, capture.end() + 1);
                        special.is_match(part)
                    },
                    None => false,
                } {
                    out += capture.as_str().parse::<u64>().unwrap();
                }
            }
        }

        Answer::from(out)
    }

    fn part_2(&self, input: &str) -> Answer {
        let lines: Vec<&str> = input.split("\n").collect();

        let mut out: u64 = 0;

        let gear = Regex::new(r"\*").unwrap();
        let num = Regex::new(r"\d+").unwrap();

        for (i, line) in lines.iter().enumerate() {
            for star in gear.find_iter(line) {
                let prev = if i > 0 { lines.get(i - 1) } else { None };
                let curr = lines.get(i);
                let next = lines.get(i+1);

                let mut nums: Vec<u64> = vec![];

                if prev != None {
                    for number in num.find_iter(prev.unwrap()) {
                        if ((1.max(number.start())-1)..(number.end()+1)).contains(&star.start()) { nums.push(number.as_str().parse().unwrap()) }
                    }
                }
                if curr != None {
                    for number in num.find_iter(curr.unwrap()) {
                        if ((1.max(number.start())-1)..(number.end()+1)).contains(&star.start()) { nums.push(number.as_str().parse().unwrap()) }
                    }
                }
                if next != None {
                    for number in num.find_iter(next.unwrap()) {
                        if ((1.max(number.start())-1)..(number.end()+1)).contains(&star.start()) { nums.push(number.as_str().parse().unwrap()) }
                    }
                }

                if nums.len() == 2 {
                    out += nums[0] * nums[1]
                }
            }
        }

        Answer::from(out)
    }
}
