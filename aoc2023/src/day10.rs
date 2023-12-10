use std::collections::HashSet;
use std::process::abort;
use shared::{Solution, Answer};

pub struct Day10;

impl Solution for Day10 {
    fn part_1(&self, input: &str) -> Answer {
        let width: i64 = input.lines().next().unwrap().len() as i64;
        let grid: Vec<_> = input.lines().collect::<Vec<&str>>().join("").chars().map(Pipes::from).collect();

        let animal = grid.iter().position(|x| x == &Pipes::Animal).unwrap();

        let mut seen: HashSet<usize> = HashSet::new();

        let mut len = 2;
        let mut current ;

        if grid[animal + 1] == Pipes::Horizontal || grid[animal + 1] == Pipes::EtoS || grid[animal + 1] == Pipes::NtoE {
            current = animal + 1;
        } else if grid[animal - 1] == Pipes::Horizontal || grid[animal - 1] == Pipes::WtoN || grid[animal - 1] == Pipes::StoW {
            current = animal - 1;
        } else { current = animal + width as usize }

        seen.insert(animal);
        seen.insert(current);

        loop {
            let next = traverse(&grid, current as i64, width);
            if next.is_none() { panic!() }
            let (next1, next2) = next.unwrap();
            if seen.contains(&next1) && seen.contains(&next2) {
                break;
            } else if !seen.contains(&next1) {
                current = next1;
                seen.insert(next1);
                len += 1;
            } else if !seen.contains(&next2) {
                current = next2;
                seen.insert(next2);
                len += 1;
            } else { panic!() }
        }

        Answer::from(len)
    }

    fn part_2(&self, input: &str) -> Answer {
        let width: usize = input.lines().next().unwrap().len();
        let mut grid: Vec<_> = input.lines().collect::<Vec<&str>>().join("").chars().map(Pipes::from).collect();

        let animal = grid.iter().position(|x| x == &Pipes::Animal).unwrap();
        replace_animal(&mut grid, animal, width);
        println!("{:?}", grid[animal]);

        let mut seen: HashSet<usize> = HashSet::new();

        let mut len = 2;
        let mut current ;

        if grid[animal + 1] == Pipes::Horizontal || grid[animal + 1] == Pipes::EtoS || grid[animal + 1] == Pipes::NtoE {
            current = animal + 1;
        } else if grid[animal - 1] == Pipes::Horizontal || grid[animal - 1] == Pipes::WtoN || grid[animal - 1] == Pipes::StoW {
            current = animal - 1;
        } else { current = animal + width as usize }

        seen.insert(animal);
        seen.insert(current);

        loop {
            let next = traverse(&grid, current as i64, width as i64);
            if next.is_none() { panic!() }
            let (next1, next2) = next.unwrap();
            if seen.contains(&next1) && seen.contains(&next2) {
                break;
            } else if !seen.contains(&next1) {
                current = next1;
                seen.insert(next1);
                len += 1;
            } else if !seen.contains(&next2) {
                current = next2;
                seen.insert(next2);
            } else { panic!() }
        }

        let mut parity = grid.chunks(width).map(|_| [0].repeat(width)).collect::<Vec<_>>();

        for (i, line) in grid.chunks(width).enumerate() {
            for (j, letter) in line.iter().enumerate() {
                if seen.contains(&(width * i + j)) {
                    for k in j..width {
                        parity[i][k] += match letter {
                            Pipes::Vertical => 2, // using 2 and 1 instead of 1 and .5 for half parity, because integers
                            Pipes::StoW => 1,
                            Pipes::NtoE => 1,
                            Pipes::EtoS => -1,
                            Pipes::WtoN => -1,
                            _ => 0
                        }
                    }
                }
            }
        }
        for i in seen {
            parity[i / width][i % width] = 0;
        }

        let mut enclosed = 0;

        for line in parity {
            enclosed += line.iter().map(|x| (x % 4 / 2)).map(i32::abs).sum::<i32>();
            println!("{:?}", line);
        }

        Answer::from(enclosed)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Pipes {
    Ground,
    Vertical,
    Horizontal,
    WtoN,
    NtoE,
    EtoS,
    StoW,
    Animal
}

impl From<char> for Pipes {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipes::Vertical,
            '-' => Pipes::Horizontal,
            'J' => Pipes::WtoN,
            'L' => Pipes::NtoE,
            'F' => Pipes::EtoS,
            '7' => Pipes::StoW,
            '.' => Pipes::Ground,
            'S' => Pipes::Animal,
            _ => unreachable!()
        }
    }
}

fn traverse(grid: &Vec<Pipes>, current: i64, width: i64) -> Option<(usize, usize)> {
    let next1 = current + match grid[current as usize] {
        Pipes::Vertical => -width,
        Pipes::Horizontal => -1,
        Pipes::WtoN => -1,
        Pipes::NtoE => -width,
        Pipes::EtoS => 1,
        Pipes::StoW => width,
        Pipes::Animal => 0,
        Pipes::Ground => unreachable!()
    };
    let next2 = current + match grid[current as usize] {
        Pipes::Vertical => width,
        Pipes::Horizontal => 1,
        Pipes::WtoN => -width,
        Pipes::NtoE => 1,
        Pipes::EtoS => width,
        Pipes::StoW => -1,
        Pipes::Animal => 0,
        Pipes::Ground => unreachable!()
    };

    // println!("{next1}, {next2}: Current={current}, Width={width}");

    // if next1 - current == -1 || next1 - current == width || next1 == -1 || next1 == (grid.len() as i64) { return None };
    // if next2 - current == -1 || next2 - current == width || next2 == -1 || next2 == (grid.len() as i64) { return None };

    Some((next1 as usize, next2 as usize))
}

fn replace_animal(grid: &mut Vec<Pipes>, animal: usize, width: usize) {
    let east = grid[animal + 1] == Pipes::Horizontal || grid[animal + 1] == Pipes::WtoN || grid[animal + 1] == Pipes::StoW;
    let north = grid[animal - width] == Pipes::Vertical || grid[animal - width] == Pipes::WtoN || grid[animal - width] == Pipes::NtoE;
    let west = grid[animal - 1] == Pipes::Horizontal || grid[animal - 1] == Pipes::WtoN || grid[animal - 1] == Pipes::StoW;
    let south = grid[animal + width] == Pipes::Vertical || grid[animal + width] == Pipes::StoW || grid[animal + width] == Pipes::EtoS;

    match (east, north, south, west) {
        (false, false, true, true) => grid[animal] = Pipes::StoW,
        (false, true, true, false) => grid[animal] = Pipes::Vertical,
        (true, true, false, false) => grid[animal] = Pipes::NtoE,
        (false, true, false, true) => grid[animal] = Pipes::WtoN,
        (true, false, true, false) => grid[animal] = Pipes::EtoS,
        (true, false, false, true) => grid[animal] = Pipes::Horizontal,
        _ => unreachable!()
    }
}

