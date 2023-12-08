use shared::Solution;

use aoc2023;
use shared::{load_actual};

use std::time::Instant;

fn main() {
    let now = Instant::now();

    let Ok(data) = load_actual(2023, 7)
        else { return; };

    let result = aoc2023::day07::Day07.part_1(&data);

    let elapsed = now.elapsed();

    println!("{}", result);

    println!("{:?}", elapsed);
}