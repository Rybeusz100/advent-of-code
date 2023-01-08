use std::{fs::File, io::BufReader};

pub mod aoc_2022_1;
pub mod aoc_2022_2;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct Answer {
    part_1: u32,
    part_2: u32,
}

fn get_file_reader(filename: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}
