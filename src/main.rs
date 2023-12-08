use shared::Solution;

use aoc2023;
use shared::{load_actual};

use std::time::Instant;

fn main() {
    let now = Instant::now();

    let Ok(data) = load_actual(2023, 4)
        else { return; };

    let result = aoc2023::day04::Day04.part_2(&data);

    let elapsed = now.elapsed();

    println!("{}", result);

    println!("{:?}", elapsed);
}