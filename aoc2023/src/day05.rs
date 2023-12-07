use std::fmt::Debug;
use shared::{Solution, Answer};

pub struct Day05;

impl Solution for Day05 {
    fn part_1(&self, input: &str) -> Answer {
        let blocks: Vec<&str> = input.split("\n\n")
            .collect();

        let seeds: Vec<u64> = blocks[0].split(" ")
            .skip(1)
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let rules: Vec<Vec<Vec<u64>>> = blocks.iter()
            .skip(1)
            .map(|rule_block|
                rule_block.split("\n")
                    .skip(1)
                    .map(|rule| rule.split(" ").map( |y|
                        y.parse::<u64>().unwrap()).collect()
                    ).collect()
            ).collect();

        let final_seeds: Vec<u64> = seeds.iter()
            .map(|seed|
                {
                    let mut res = seed.clone();
                    for rule in &rules {
                        res = map(res, &rule);
                    }
                    res
                }
            ).collect();

        Answer::from(final_seeds.iter().min().unwrap().clone())
    }

    fn part_2(&self, input: &str) -> Answer {
        todo!()
    }
}

fn map(seed: u64, rules: &Vec<Vec<u64>>) -> u64 {
    for rule in rules {
        let dest = rule[0];
        let source = rule[1];
        let range = rule[2];

        if (source..(source+range)).contains(&seed) {
            return seed - source + dest
        }
    }
    seed
}
