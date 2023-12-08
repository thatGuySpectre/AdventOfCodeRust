use std::collections::HashMap;
use std::iter::repeat;
use regex::Regex;
use shared::{Solution, Answer};

pub struct Day08;

impl Solution for Day08 {
    fn part_1(&self, input: &str) -> Answer {
        let re = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();

        let instructions = input.split_once("\n\n").unwrap().0;
        let directions: HashMap<&str, (&str, &str)> = input.split_once("\n\n")
            .unwrap()
            .1
            .split("\n")
            .map(|line| {
                let caps = re.captures(line).unwrap();
                (caps.get(1).unwrap().as_str(), (caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str()))
            })
            .collect::<HashMap<&str, (&str, &str)>>();

        let mut steps = 0;
        let mut curr = "AAA";

        for inst in instructions.chars().cycle() {
            steps += 1;
            curr = match inst {
                'L' => directions.get(curr).unwrap().0,
                'R' => directions.get(curr).unwrap().1,
                _ => panic!("invalid instruction")
            };
            if curr == "ZZZ" {
                return Answer::from(steps)
            }
        }

        Answer::Unimplemented
    }

    fn part_2(&self, input: &str) -> Answer {
        let re = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();

        let instructions = input.split_once("\n\n").unwrap().0;
        let directions: HashMap<&str, (&str, &str)> = input.split_once("\n\n")
            .unwrap()
            .1
            .split("\n")
            .map(|line| {
                let caps = re.captures(line).unwrap();
                (caps.get(1).unwrap().as_str(), (caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str()))
            })
            .collect::<HashMap<&str, (&str, &str)>>();

        let mut curr: Vec<&str> = directions.keys()
            .filter_map(|x| if x.chars().last() == Some('A') { Some(*x) } else { None })
            .collect();

        let mut steps: u64 = 0;

        let mut period: Vec<Option<u64>> = repeat(None).take(curr.len()).collect();

        for inst in instructions.chars().cycle() {
            steps += 1;
            curr = curr.iter()
                .map(|x| match inst {
                    'L' => directions.get(x).unwrap().0,
                    'R' => directions.get(x).unwrap().1,
                    _ => panic!("invalid instruction")
                })
                .collect();

            for i in 0..curr.len() {
               if (period[i] == None && curr[i].chars().last() == Some('Z') ) {
                   period[i] = Some(steps);
               }
            }

            if period.iter().all(|x| *x != None) {
                break
            }
        }

        let mut out: u64 = period[0].unwrap();
        let mut last = out;

        for num in period.iter().skip(1) {
            out *= num.unwrap();
            out /= gcd(last, num.unwrap());
            last = num.unwrap();
        }

        Answer::from(out)
    }
}

// Shamelessly copied from https://rustycloud.org/_code_samples/gcd.html
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // Make sure that either number isn't 0
    assert!(n != 0 && m != 0);

    // Run a loop while the remainder `m` is not 0
    while m != 0 {
        // Make sure we are always trying to get the divsor of
        // the smaller number
        if m < n {
            // initialize a temp variable
            let temp = m;
            m = n;
            n = temp;
        }
        // Get the divisor
        m = m % n;
    }
    // return n aka the greatest common denominator
    n
}