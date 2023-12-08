use std::cmp::Ordering;
use std::collections::{HashMap};
use std::iter::zip;
use shared::{Solution, Answer};

pub struct Day07;

impl Solution for Day07 {
    fn part_1(&self, input: &str) -> Answer {
        let lines: Vec<&str> = input.split("\n").collect();
        let mut hands: Vec<(&str, u64)> = lines.iter().map(|l|
            {
                let x = l.split(" ").collect::<Vec<&str>>();
                (x[0], x[1].parse::<u64>().unwrap())
            }
        ).collect();

        hands.sort_by(compare);

        let res: u64 = hands.iter()
            .enumerate()
            .map(|(i, (hand, bet))| ((i + 1) as u64) * bet
            ).sum();

        Answer::from(res)
    }

    fn part_2(&self, input: &str) -> Answer {
        let lines: Vec<&str> = input.split("\n").collect();
        let mut hands: Vec<(&str, u64)> = lines.iter().map(|l|
            {
                let x = l.split(" ").collect::<Vec<&str>>();
                (x[0], x[1].parse::<u64>().unwrap())
            }
        ).collect();

        hands.sort_by(crate::day07::compare2);

        let res: u64 = hands.iter()
            .enumerate()
            .map(|(i, (hand, bet))| ((i + 1) as u64) * bet
            ).sum();

        Answer::from(res)
    }
}

fn compare(hand1: &(&str, u64), hand2: &(&str, u64)) -> Ordering {
    let mut s: HashMap<char, u64> = HashMap::new();
    let mut o: HashMap<char, u64> = HashMap::new();

    for c in hand1.0.chars() {
        s.insert(c, match s.get(&c) {
            Some(v) => v+1,
            None => 1
        });
    }

    for c in hand2.0.chars() {
        o.insert(c, match o.get(&c) {
            Some(v) => v+1,
            None => 1
        });
    }

    if ( s.keys().count() < o.keys().count() ) |
        ( s.values().max() > o.values().max() ) { return Ordering::Greater }
    else if ( s.keys().count() > o.keys().count() ) |
        ( s.values().max() < o.values().max() ) { return Ordering::Less }

    let ordering = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

    for (i, j) in zip(hand1.0.chars(), hand2.0.chars()) {
        if ordering.iter().position(|&x| x==i) < ordering.iter().position(|&x| x==j) {
            return Ordering::Greater
        } else if ordering.iter().position(|&x| x==i) > ordering.iter().position(|&x| x==j) {
            return Ordering::Less
        }
    }

    return Ordering::Equal
}

fn compare2(hand1: &(&str, u64), hand2: &(&str, u64)) -> Ordering {
    let mut s: HashMap<char, u64> = HashMap::new();
    let mut o: HashMap<char, u64> = HashMap::new();

    for c in hand1.0.chars() {
        s.insert(c, match s.get(&c) {
            Some(v) => v+1,
            None => 1
        });
    }

    for c in hand2.0.chars() {
        o.insert(c, match o.get(&c) {
            Some(v) => v+1,
            None => 1
        });
    }

    if o.contains_key(&'J') {
        let val = o[&'J'];
        o.remove(&'J');
        let max_key = o.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap_or((&'A', &0)).0;
        o.insert(*max_key, match o.get(&max_key) {
            Some(x) => x,
            None => &0
        } + val);
    }

    if s.contains_key(&'J') {
        let val = s[&'J'];
        s.remove(&'J');
        let max_key = s.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap_or((&'A', &0)).0;
        s.insert(*max_key, match s.get(&max_key) {
            Some(x) => x,
            None => &0
        } + val);
    }

    if ( s.keys().count() < o.keys().count() ) |
        ( s.values().max() > o.values().max() ) { return Ordering::Greater }
    else if ( s.keys().count() > o.keys().count() ) |
        ( s.values().max() < o.values().max() ) { return Ordering::Less }

    let ordering = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

    for (i, j) in zip(hand1.0.chars(), hand2.0.chars()) {
        if ordering.iter().position(|&x| x==i) < ordering.iter().position(|&x| x==j) {
            return Ordering::Greater
        } else if ordering.iter().position(|&x| x==i) > ordering.iter().position(|&x| x==j) {
            return Ordering::Less
        }
    }

    return Ordering::Equal
}
