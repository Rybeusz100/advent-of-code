// https://adventofcode.com/2022/day/8

use std::io::BufRead;

use super::{get_file_reader, Answer};

pub fn solution() -> std::io::Result<Answer<usize>> {
    let reader = get_file_reader("input/aoc_2022/aoc_2022_8.txt")?;

    let mut heights: Vec<Vec<i8>> = Vec::new();
    let mut visible: Vec<Vec<bool>> = Vec::new();
    let mut scenic_scores: Vec<Vec<[usize; 4]>> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        heights.push(Vec::new());
        visible.push(Vec::new());
        scenic_scores.push(Vec::new());

        for c in line?.chars() {
            heights[i].push(c.to_digit(10).unwrap() as i8);
            visible[i].push(false);
            scenic_scores[i].push([0; 4]);
        }
    }

    // part 1
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

    // part 2
    let mut height_locations;

    for x in 0..=last_idx {
        height_locations = [[0_usize; 10], [0_usize; 10], [last_idx; 10], [last_idx; 10]];
        for y in 0..=last_idx {
            scenic_scores[x][y][0] = get_score(
                &height_locations[0],
                heights[x][y],
                y,
                Direction::LeftUp,
                last_idx,
            );
            scenic_scores[y][x][1] = get_score(
                &height_locations[1],
                heights[y][x],
                y,
                Direction::LeftUp,
                last_idx,
            );
            scenic_scores[last_idx - x][last_idx - y][2] = get_score(
                &height_locations[2],
                heights[last_idx - x][last_idx - y],
                last_idx - y,
                Direction::RightDown,
                last_idx,
            );
            scenic_scores[last_idx - y][last_idx - x][3] = get_score(
                &height_locations[3],
                heights[last_idx - y][last_idx - x],
                last_idx - y,
                Direction::RightDown,
                last_idx,
            );

            height_locations[0][heights[x][y] as usize] = y;
            height_locations[1][heights[y][x] as usize] = y;
            height_locations[2][heights[last_idx - x][last_idx - y] as usize] = last_idx - y;
            height_locations[3][heights[last_idx - y][last_idx - x] as usize] = last_idx - y;
        }
    }

    let part_2 = scenic_scores
        .into_iter()
        .flatten()
        .map(|x| x.into_iter().product())
        .max()
        .unwrap();

    Ok(Answer { part_1, part_2 })
}

fn get_score(
    height_locations: &[usize; 10],
    height: i8,
    current_loc: usize,
    dir: Direction,
    last_idx: usize,
) -> usize {
    let mut max_loc = match dir {
        Direction::LeftUp => 0,
        Direction::RightDown => last_idx,
    };
    for loc in height_locations[(height as usize)..].iter() {
        match dir {
            Direction::LeftUp => {
                if *loc > max_loc {
                    max_loc = *loc;
                }
            }
            Direction::RightDown => {
                if *loc < max_loc {
                    max_loc = *loc;
                }
            }
        }
    }
    if current_loc > max_loc {
        current_loc - max_loc
    } else {
        max_loc - current_loc
    }
}

enum Direction {
    LeftUp,
    RightDown,
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
                part_2: 595080
            }
        );
    }
}
