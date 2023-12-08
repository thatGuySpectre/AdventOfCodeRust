use std::collections::HashSet;
use shared::{Solution, Answer};

pub struct Day04;

impl Solution for Day04 {
    fn part_1(&self, input: &str) -> Answer {
        let lines: Vec<&str> = input.split("\n").collect();
        let cards = lines.iter()
            .map(|l| l.split_once(": ")
                .unwrap()
                .1
                .split_once(" | ")
                .unwrap())
            .map(|(a, b)| (
                a.split_whitespace().map(|y| y.parse::<u64>().unwrap()).collect::<HashSet<u64>>(),
                b.split_whitespace().map(|y| y.parse::<u64>().unwrap()).collect::<HashSet<u64>>()
            ))
            .collect::<Vec<(HashSet<u64>, HashSet<u64>)>>();

        let mut out: u64 = 0;

        for (card, winners) in cards {
            let winning_nums = card.intersection(&winners).count();
            if winning_nums > 0 {
                out += 2u64.pow(winning_nums as u32 - 1)
            }
        }

        Answer::from(out)
    }

    fn part_2(&self, input: &str) -> Answer {
        let lines: Vec<&str> = input.split("\n").collect();
        let cards = lines.iter()
            .map(|l| l.split_once(": ").
                unwrap()
                .1
                .split_once(" | ")
                .unwrap())
            .map(|(a, b)| (
                a.split_whitespace().map(|y| y.parse::<u64>().unwrap()).collect::<HashSet<u64>>(),
                b.split_whitespace().map(|y| y.parse::<u64>().unwrap()).collect::<HashSet<u64>>()
            ))
            .collect::<Vec<(HashSet<u64>, HashSet<u64>)>>();

        let mut nums = vec![1u64; lines.len()];

        for (i, (card, winners)) in cards.iter().enumerate() {
            let winning_nums = card.intersection(&winners).count();
            for j in (i+1)..(i+winning_nums+1) {
                nums[j] += nums[i]
            }
        }

        Answer::from(nums.iter().sum::<u64>())
    }
}
