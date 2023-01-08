// https://adventofcode.com/2022/day/3

use std::{collections::HashSet, io::BufRead};

use super::{get_file_reader, Answer};

pub fn solution() -> std::io::Result<Answer> {
    let reader = get_file_reader("input/aoc_2022/aoc_2022_3.txt")?;

    let mut priorities_sum = 0u32;

    let mut group_common_elements: HashSet<u8> = HashSet::new();
    let mut group_member_index = 0;
    let mut group_priorities_sum = 0u32;

    for line in reader.lines() {
        let priorities: Vec<u8> = line?
            .chars()
            .map(|c| {
                let mut c = c as u8;
                if c >= 97 {
                    c -= 96;
                } else {
                    c -= 38;
                }
                c
            })
            .collect();

        let (left, right) = priorities.split_at(priorities.len() / 2);
        let left_set: HashSet<u8> = left.iter().cloned().collect();
        let right_set: HashSet<u8> = right.iter().cloned().collect();
        let common: HashSet<&u8> = left_set.intersection(&right_set).collect();
        let union = left_set.union(&right_set).cloned().collect();

        if group_member_index == 0 {
            group_common_elements = union;
        } else {
            group_common_elements = union
                .intersection(&group_common_elements)
                .cloned()
                .collect();
        }
        group_member_index += 1;

        if group_member_index == 3 {
            group_member_index = 0;
            group_priorities_sum += group_common_elements
                .iter()
                .map(|val| *val as u32)
                .last()
                .unwrap();
        }

        priorities_sum += common.iter().map(|val| **val as u32).last().unwrap();
    }

    Ok(Answer {
        part_1: priorities_sum,
        part_2: group_priorities_sum,
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
                part_1: 8139,
                part_2: 2668
            }
        );
    }
}
