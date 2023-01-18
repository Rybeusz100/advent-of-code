use std::time::Instant;

mod aoc_2022;
use aoc_2022::*;

macro_rules! time_and_print {
    ($message:expr, $func:expr) => {
        let start = Instant::now();
        let result = $func;
        let end = Instant::now();
        println!("{} in {:?}", $message, end.duration_since(start));
        println!("{:#?}", result);
        println!();
    };
}

fn main() {
    time_and_print!("AoC 2022 day 1", aoc_2022_1::solution().unwrap());
    time_and_print!("AoC 2022 day 2", aoc_2022_2::solution().unwrap());
    time_and_print!("AoC 2022 day 3", aoc_2022_3::solution().unwrap());
    time_and_print!("AoC 2022 day 4", aoc_2022_4::solution().unwrap());
    time_and_print!("AoC 2022 day 5", aoc_2022_5::solution().unwrap());
    time_and_print!("AoC 2022 day 6", aoc_2022_6::solution().unwrap());
}
