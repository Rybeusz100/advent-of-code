// https://adventofcode.com/2022/day/4

use std::io::BufRead;

use super::{get_file_reader, Answer};

pub fn solution() -> std::io::Result<Answer<u32>> {
    let reader = get_file_reader("input/aoc_2022/aoc_2022_4.txt")?;

    let mut fully_contained = 0u32;
    let mut overlaps = 0u32;

    for line in reader.lines() {
        let line = line?;
        let left_right: Vec<&str> = line.split(',').collect();
        let left_right: Vec<Vec<u32>> = left_right
            .iter()
            .map(|v| v.split('-').map(|c| c.parse().unwrap()).collect())
            .collect();
        let (left, right) = (&left_right[0], &left_right[1]);

        if (left[0] >= right[0] && left[1] <= right[1])
            || (left[0] <= right[0] && left[1] >= right[1])
        {
            fully_contained += 1;
            overlaps += 1;
        } else if (left[0] >= right[0] && left[0] <= right[1])
            || (left[1] >= right[0] && left[1] <= right[1])
        {
            overlaps += 1;
        }
    }

    Ok(Answer {
        part_1: fully_contained,
        part_2: overlaps,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let result = solution().unwrap();
        assert_eq!(
            result,
            Answer {
                part_1: 464,
                part_2: 770
            }
        );
    }
}
