use std::{fs, io};

pub fn load_actual(year: u64, day: u64) -> io::Result<String> {
    let path = format!("./data/{year}/day{:02}", day);
    fs::read_to_string(path)
}

pub fn load_test(year: u64, day: u64) -> io::Result<String> {
    let path = format!("./examples/{year}/day{:02}", day);
    fs::read_to_string(path)
}