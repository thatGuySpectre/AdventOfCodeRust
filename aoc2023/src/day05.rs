use std::cmp::{max, min};
use std::fmt::{Debug, Display, Formatter};
use shared::{Solution, Answer};

pub struct Day05;

impl Solution for Day05 {
    fn part_1(&self, input: &str) -> Answer {
        let blocks: Vec<&str> = input.split("\n\n")
            .collect();

        let seeds: Vec<i64> = blocks[0].split(" ")
            .skip(1)
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let rules: Vec<Vec<Vec<i64>>> = blocks.iter()
            .skip(1)
            .map(|rule_block|
                rule_block.split("\n")
                    .skip(1)
                    .map(|rule| rule.split(" ").map( |y|
                        y.parse::<i64>().unwrap()).collect()
                    ).collect()
            ).collect();

        let final_seeds: Vec<i64> = seeds.iter()
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
        let blocks: Vec<&str> = input.split("\n\n")
            .collect();

        let mut seeds: Vec<SeedRange> = blocks[0].split(" ")
            .skip(1)
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|x| SeedRange::from_seeds(x[0].parse::<i64>().unwrap(), x[1].parse::<i64>().unwrap()))
            .collect();

        let rule_blocks: Vec<Vec<Rule>> = blocks.iter()
            .skip(1)
            .map(
                |block| block.split("\n").skip(1).map(
                    |line| {
                        let l: Vec<&str> = line.split(" ").collect();
                        Rule::from_rules(l[0].parse::<i64>().unwrap(), l[1].parse::<i64>().unwrap(), l[2].parse::<i64>().unwrap())
                    }
                ).collect()
            ).collect();


        for rule_blk in &rule_blocks {
            let mut ranges_to_check: Vec<SeedRange> = seeds.clone();
            let mut next_ranges: Vec<SeedRange> = vec![];
            let mut ranges_done: Vec<SeedRange> = vec![];

            for rule in rule_blk {
                for range in ranges_to_check {
                    match rule.apply(&range) {
                        None => next_ranges.push(range),
                        Some((None, mid, None)) => ranges_done.push(mid),
                        Some((Some(first), mid, None)) => { next_ranges.push(first); ranges_done.push(mid) },
                        Some((None, mid, Some(last))) => { next_ranges.push(last); ranges_done.push(mid) },
                        Some((Some(first), mid, Some(last))) => { next_ranges.push(first); next_ranges.push(last); ranges_done.push(mid) }
                    }
                }
                ranges_to_check = next_ranges;
                next_ranges = vec![];
            }

            seeds = [ranges_to_check, ranges_done].concat();
        }

        let result = seeds.iter().min_by_key(|x| x.start).unwrap().start;

        Answer::from(result)
    }
}

fn map(seed: i64, rules: &Vec<Vec<i64>>) -> i64 {
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

#[derive(Clone, Debug)]
struct SeedRange {
    start: i64,
    end: i64,
}

impl SeedRange {
    fn new(start: i64, end: i64) -> SeedRange {
        SeedRange {start, end}
    }

    fn from_seeds(start: i64, range: i64) -> SeedRange {
        SeedRange {start, end: start + range - 1}
    }
}

impl Display for SeedRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[derive(Clone, Debug)]
struct Rule {
    start: i64,
    end: i64,
    offset: i64,
}

impl Rule {
    fn new(start: i64, end: i64, offset: i64) -> Rule {
        Rule {start, end, offset}
    }

    fn from_rules(dest: i64, source: i64, range: i64) -> Rule {
        Rule {start: source, end: source + range - 1, offset: (dest - source) as i64}
    }

    fn apply(&self, seeds: &SeedRange) -> Option<(Option<SeedRange>, SeedRange, Option<SeedRange>)> {
        if (self.start <= seeds.end && self.end >= seeds.start) {
            Some(
                (
                    if self.start > seeds.start { Some(SeedRange::new(seeds.start, self.start - 1)) } else { None },
                    SeedRange::new(max(self.start, seeds.start) + self.offset, min(self.end, seeds.end) + self.offset),
                    if self.end < seeds.end { Some(SeedRange::new(self.end + 1, seeds.end)) } else { None },
                )
            )
        }
        else { None }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}: {}", self.start, self.end, self.offset)
    }
}

