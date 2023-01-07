// https://adventofcode.com/2022/day/2

use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Answer {
    part_1: u32,
    part_2: u32,
}

pub fn solution() -> std::io::Result<Answer> {
    let file = File::open("input/aoc_2022/aoc_2022_2.txt")?;
    let mut reader = BufReader::new(file);

    // part 1
    let mut score = 0u32;
    for line in reader.by_ref().lines() {
        let line = line?;
        let mut shapes: Vec<&str> = line.split(' ').collect();
        shapes[1] = match shapes[1] {
            "X" => "A",
            "Y" => "B",
            "Z" => "C",
            _ => shapes[1],
        };

        score += calculate_score(shapes);
    }

    reader.seek(SeekFrom::Start(0))?;
    // part 2
    let mut score_2 = 0u32;
    for line in reader.lines() {
        let line = line?;
        let mut shapes: Vec<&str> = line.split(' ').collect();
        shapes[1] = match shapes[..] {
            ["A", "Z"] | ["B", "Y"] | ["C", "X"] => "B",
            ["A", "Y"] | ["B", "X"] | ["C", "Z"] => "A",
            ["A", "X"] | ["B", "Z"] | ["C", "Y"] => "C",
            _ => shapes[1],
        };

        score_2 += calculate_score(shapes);
    }

    Ok(Answer {
        part_1: score,
        part_2: score_2,
    })
}

fn calculate_score(shapes: Vec<&str>) -> u32 {
    let mut score = 0u32;
    match shapes[1] {
        "A" => score += 1,

        "B" => score += 2,

        "C" => score += 3,
        _ => (),
    };

    match shapes[..] {
        [x, y] if x == y => score += 3,
        ["A", "B"] | ["B", "C"] | ["C", "A"] => score += 6,
        _ => (),
    }

    score
}
