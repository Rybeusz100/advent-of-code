// https://adventofcode.com/2022/day/6

use std::{collections::HashSet, io::BufRead};

use super::{get_file_reader, Answer};

pub fn solution() -> std::io::Result<Answer<usize>> {
    let mut reader = get_file_reader("input/aoc_2022/aoc_2022_6.txt")?;

    let mut message = String::new();
    reader.read_line(&mut message)?;

    Ok(Answer {
        part_1: find_start(&message, 4),
        part_2: find_start(&message, 14),
    })
}

fn find_start(message: &str, unique_count: usize) -> usize {
    let mut last_n: Vec<char> = Vec::new();

    for c in message.chars().take(unique_count) {
        last_n.push(c);
    }

    let mut unique: HashSet<char>;

    for (i, c) in message.chars().skip(unique_count).enumerate() {
        unique = last_n.iter().copied().collect();
        if unique.len() == unique_count {
            return i + unique_count;
        }
        last_n.remove(0);
        last_n.push(c);
    }

    0
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
                part_1: 1210,
                part_2: 3476
            }
        );
    }
}
