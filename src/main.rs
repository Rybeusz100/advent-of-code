use std::time::Instant;

mod aoc_2022;
use aoc_2022::*;

macro_rules! aoc {
    ($year:literal, $day:literal, $mod_name:ident) => {
        let start = Instant::now();
        let result = $mod_name::solution().unwrap();
        let end = Instant::now();
        println!(
            "AoC {} day {} in {:?}\n{:#?}\n",
            $year,
            $day,
            end.duration_since(start),
            result
        );
    };
}

fn main() {
    aoc!(2022, 1, aoc_2022_1);
    aoc!(2022, 2, aoc_2022_2);
    aoc!(2022, 3, aoc_2022_3);
    aoc!(2022, 4, aoc_2022_4);
    aoc!(2022, 5, aoc_2022_5);
    aoc!(2022, 6, aoc_2022_6);
    aoc!(2022, 7, aoc_2022_7);
    aoc!(2022, 8, aoc_2022_8);
}
