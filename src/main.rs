use shared::Solution;

use aoc2023;
use shared::{load_actual};

use std::time::Instant;

fn main() {
    let now = Instant::now();

    let Ok(data) = load_actual(2023, 11)
        else { panic!("No Input Data"); };

    let result = aoc2023::day11::Day11.part_2(&data);

    let elapsed = now.elapsed();

    println!("{}", result);

    println!("{:?}", elapsed);
}