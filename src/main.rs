use shared::Solution;
mod load_data;

use aoc2023;
use load_data::load;

fn main() {
    let Ok(data) = load(2023, 2)
        else { return; };

    print!("{}", aoc2023::day02::Day02.part_2(&data));
}