use std::collections::HashSet;
use shared::{Solution, Answer};

pub struct Day10;

impl Solution for Day10 {
    fn part_1(&self, input: &str) -> Answer {
        let mut grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let full_animal = grid.iter().flatten().position(|&x| x == 'S').unwrap();
        let animal = (full_animal / grid.len(), full_animal % grid.len());

        replace_animal(&mut grid, &animal);

        let mut pos = animal;
        let mut last: Option<(usize, usize)> = None;

        let mut len_of_loop = 0;

        loop {
            len_of_loop += 1;
            let tmp = pos;
            pos = traverse(&grid[pos.0][pos.1], &pos, &last);
            last = Some(tmp);

            if pos == animal {break}
        }

        Answer::from(len_of_loop / 2)
    }

    fn part_2(&self, input: &str) -> Answer {
        let mut grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let full_animal = grid.iter().flatten().position(|&x| x == 'S').unwrap();
        let animal = (full_animal / grid.len(), full_animal % grid.len());

        replace_animal(&mut grid, &animal);

        let mut pos = animal;
        let mut last: Option<(usize, usize)> = None;

        let mut in_loop: HashSet<(usize, usize)> = HashSet::new();

        loop {
            in_loop.insert(pos);
            let tmp = pos;
            pos = traverse(&grid[pos.0][pos.1], &pos, &last);
            last = Some(tmp);

            if pos == animal {break}
        }

        let mut total = 0;

        for (i, line) in grid.iter().enumerate() {
            let mut parity = 0;
            for (j, letter) in line.iter().enumerate() {
                if in_loop.contains(&(i, j)) {
                    parity += modify_parity(letter)
                }
                else if parity % 4 != 0 {
                    total += 1
                }
            }
        }

        Answer::from(total)
    }
}

fn traverse(pipe: &char, pos: &(usize, usize), last: &Option<(usize, usize)>) -> (usize, usize) {
    let (offset1, offset2) = match *pipe {
        '|' => ((-1, 0), (1, 0)),
        '-' => ((0, -1), (0, 1)),
        'J' => ((-1, 0), (0, -1)),
        'L' => ((-1, 0), (0, 1)),
        'F' => ((1, 0), (0, 1)),
        '7' => ((1, 0), (0, -1)),
        l => panic!("Invalid letter {l}")
    };

    let new1 = ((offset1.0 + pos.0 as i32) as usize, (offset1.1 + pos.1 as i32) as usize);
    let new2 = ((offset2.0 + pos.0  as i32) as usize, (offset2.1 + pos.1 as i32) as usize);

    match last {
        None => new1,
        Some(p) if p == &new1 => new2,
        Some(p) if p == &new2 => new1,
        _ => unreachable!()
    }
}

fn replace_animal(grid: &mut Vec<Vec<char>>, pos: &(usize, usize)) {
    let left = match grid.get(pos.0) { Some(x) => x.get((pos.1 as i32 - 1) as usize), _ => None};
    let right = match grid.get(pos.0) { Some(x) => x.get(pos.1 + 1), _ => None};
    let top = match grid.get((pos.0 as i32 - 1) as usize) { Some(x) => x.get(pos.1), _ => None};
    let bottom = match grid.get(pos.0 + 1) { Some(x) => x.get(pos.1), _ => None};

    grid[pos.0][pos.1] = match (left, right, top, bottom) {
        (Some(x), Some(y), _, _) if ['-', 'L', 'F'].contains(x) && ['-', 'J', '7'].contains(y) => '-',
        (Some(x), _, Some(y), _) if ['-', 'L', 'F'].contains(x) && ['|', '7', 'F'].contains(y) => 'J',
        (Some(x), _, _, Some(y)) if ['-', 'L', 'F'].contains(x) && ['|', 'J', 'L'].contains(y) => '7',
        (_, Some(x), Some(y), _) if ['-', 'J', '7'].contains(x) && ['|', '7', 'F'].contains(y) => 'L',
        (_, Some(x), _, Some(y)) if ['-', 'J', '7'].contains(x) && ['|', 'J', 'L'].contains(y) => 'F',
        (_, _, Some(x), Some(y)) if ['|', '7', 'F'].contains(x) && ['|', 'J', 'L'].contains(y) => '|',
        _ => unreachable!()
    }
}

fn modify_parity(l: &char) -> i32 {
    match l {
        '|' => 2,
        'L' => 1,
        'J' => -1,
        'F' => -1,
        '7' => 1,
        _ => 0
    }
}
