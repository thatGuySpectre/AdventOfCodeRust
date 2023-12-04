use shared::{Answer, Solution};
use regex::{Regex};

pub struct Day01;

impl Solution for Day01 {
    fn part_1(&self, input: &str) -> Answer {
        let words: Vec<&str> = input.split("\n")
            .collect();

        let mut sum: u64 = 0;

        for word in words {
            let mut first: Option<u64> = None;
            let mut last: Option<u64> = None;
            for char in word.chars() {
                if char.is_digit(10) {
                    last = Some(char.to_digit(10).unwrap().into());
                    if first == None {
                        first = Some(char.to_digit(10).unwrap().into());
                    }
                }
            }

            sum += 10 * first.unwrap() + last.unwrap()
        }

        Answer::from(sum)
    }

    fn part_2(&self, input: &str) -> Answer {
        let words: Vec<&str> = input.split("\n")
            .collect();

        let first = Regex::new(
            r".*?(?:(1|one)|(2|two)|(3|three)|(4|four)|(5|five)|(6|six)|(7|seven)|(8|eight)|(9|nine)|(0|zero)).*"
        ).unwrap();
        let last = Regex::new(
            r".*(?:(1|one)|(2|two)|(3|three)|(4|four)|(5|five)|(6|six)|(7|seven)|(8|eight)|(9|nine)|(0|zero)).*?"
        ).unwrap();

        Answer::from(words.iter()
            .map(|word| {10 * matches(&first.captures(&word).unwrap()) + matches(&last.captures(&word).unwrap())})
            .collect::<Vec<_>>()
            .iter()
            .sum::<u64>())
    }
}

fn matches(cap: &regex::Captures) -> u64 {
    for i in 1..10 {
        match cap.get(i) {
            Some(_) => return i as u64,
            None  => (),
        };
    }
    return 0
}