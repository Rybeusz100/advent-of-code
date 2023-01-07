// https://adventofcode.com/2022/day/1

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Answer {
    part_1: u32,
    part_2: u32,
}

pub fn solution() -> std::io::Result<Answer> {
    let mut calories: Vec<u32> = vec![0];
    let file = File::open("input/aoc_2022/aoc_2022_1.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            calories.push(0);
        } else {
            *calories.last_mut().unwrap() += line.parse::<u32>().unwrap();
        }
    }

    calories.sort_unstable_by(|a, b| b.cmp(a));

    Ok(Answer {
        part_1: calories[0],
        part_2: calories[0..3].iter().sum(),
    })
}
