use shared::Solution;
mod load_data;

use aoc2023;
use load_data::load;

use std::time::Instant;

fn main() {
    let now = Instant::now();

    let Ok(data) = load(2023, 6)
        else { return; };

    let result = aoc2023::day06::Day06.part_2(&data);

    let elapsed = now.elapsed();

    println!("{}", result);

    println!("{:?}", elapsed);
}