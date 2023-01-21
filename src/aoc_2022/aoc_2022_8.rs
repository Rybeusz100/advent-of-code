// https://adventofcode.com/2022/day/8

use std::io::BufRead;

use super::{get_file_reader, Answer};

pub fn solution() -> std::io::Result<Answer<usize>> {
    let reader = get_file_reader("input/aoc_2022/aoc_2022_8.txt")?;

    let mut heights: Vec<Vec<i8>> = Vec::new();
    let mut visible: Vec<Vec<bool>> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        heights.push(Vec::new());
        visible.push(Vec::new());

        for c in line?.chars() {
            heights[i].push(c.to_digit(10).unwrap() as i8);
            visible[i].push(false);
        }
    }

    let mut tallest;

    let last_idx = heights.len() - 1;

    for x in 0..=last_idx {
        tallest = [-1; 4];
        for y in 0..=last_idx {
            if heights[x][y] > tallest[0] {
                visible[x][y] = true;
            }
            if heights[y][x] > tallest[1] {
                visible[y][x] = true;
            }
            if heights[last_idx - x][last_idx - y] > tallest[2] {
                visible[last_idx - x][last_idx - y] = true;
            }
            if heights[last_idx - y][last_idx - x] > tallest[3] {
                visible[last_idx - y][last_idx - x] = true;
            }

            if heights[x][y] > tallest[0] {
                tallest[0] = heights[x][y];
            }
            if heights[y][x] > tallest[1] {
                tallest[1] = heights[y][x];
            }
            if heights[last_idx - x][last_idx - y] > tallest[2] {
                tallest[2] = heights[last_idx - x][last_idx - y];
            }
            if heights[last_idx - y][last_idx - x] > tallest[3] {
                tallest[3] = heights[last_idx - y][last_idx - x];
            }
        }
    }

    let part_1 = visible.into_iter().flatten().filter(|x| *x).count();

    Ok(Answer { part_1, part_2: 0 })
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
                part_1: 1543,
                part_2: 0
            }
        );
    }
}
