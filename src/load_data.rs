use std::{fs, io};

pub fn load(year: u32, day:u32) -> io::Result<String> {
    let path = format!("./data/{year}/day{:02}", day);
    fs::read_to_string(path)
}