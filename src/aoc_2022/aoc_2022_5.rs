// https://adventofcode.com/2022/day/5

use std::io::{BufRead, Read};

use super::{get_file_reader, Answer};

pub fn solution() -> std::io::Result<Answer<String>> {
    let mut reader = get_file_reader("input/aoc_2022/aoc_2022_5.txt")?;

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for line in reader.by_ref().lines() {
        let line = line?;
        if line.trim().starts_with('1') {
            break;
        }

        let mut row = Vec::new();

        for i in (1..line.len() - 1).step_by(4) {
            row.push(line.chars().nth(i).unwrap());
        }

        if stacks.is_empty() {
            for _ in 0..row.len() {
                stacks.push(Vec::new());
            }
        }

        for (i, c) in row.iter().enumerate() {
            stacks[i].push(*c);
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
        while stack.last().unwrap().is_whitespace() {
            stack.pop();
        }
    }

    let mut stacks_2 = stacks.clone();

    let mut mv: [usize; 3] = [0; 3];
    for line in reader.lines() {
        let line = line?;

        for (i, word) in line.split_whitespace().enumerate() {
            if i % 2 == 0 {
                continue;
            }

            mv[(i - 1) / 2] = word.parse().unwrap();
        }

        for _ in 0..mv[0] {
            let to_move = stacks[mv[1] - 1].pop().unwrap();
            stacks[mv[2] - 1].push(to_move);
        }

        // part 2
        let mut temp = Vec::new();
        for _ in 0..mv[0] {
            temp.push(stacks_2[mv[1] - 1].pop().unwrap());
        }
        for el in temp.iter().rev() {
            stacks_2[mv[2] - 1].push(*el);
        }
    }

    let mut answer = Answer {
        part_1: "".to_owned(),
        part_2: "".to_owned(),
    };

    for stack in stacks.iter() {
        answer.part_1.push(*stack.last().unwrap());
    }

    for stack in stacks_2.iter() {
        answer.part_2.push(*stack.last().unwrap());
    }

    Ok(answer)
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
                part_1: "BSDMQFLSP".to_owned(),
                part_2: "PGSQBFLDP".to_owned()
            }
        );
    }
}
