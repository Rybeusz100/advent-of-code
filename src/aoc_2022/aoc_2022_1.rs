// https://adventofcode.com/2022/day/1

use std::io::BufRead;

use super::{get_file_reader, Answer};

pub fn solution() -> std::io::Result<Answer> {
    let reader = get_file_reader("input/aoc_2022/aoc_2022_1.txt")?;
    let mut calories: Vec<u32> = vec![0];

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
