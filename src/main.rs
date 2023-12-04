use shared::Solution;
mod load_data;

use aoc2023;
use load_data::load;

fn main() {
    let Ok(data) = load(2023, 1)
        else { return; };

    print!("{}", aoc2023::day02::Day02.part_1(&data));
}