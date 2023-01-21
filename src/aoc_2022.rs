use std::{fs::File, io::BufReader};

pub mod aoc_2022_1;
pub mod aoc_2022_2;
pub mod aoc_2022_3;
pub mod aoc_2022_4;
pub mod aoc_2022_5;
pub mod aoc_2022_6;
pub mod aoc_2022_7;
pub mod aoc_2022_8;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct Answer<T> {
    part_1: T,
    part_2: T,
}

fn get_file_reader(filename: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}
