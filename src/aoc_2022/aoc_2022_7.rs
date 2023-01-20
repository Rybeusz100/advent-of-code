// https://adventofcode.com/2022/day/7

use std::{
    borrow::Borrow,
    cell::RefCell,
    io::BufRead,
    rc::{Rc, Weak},
};

use super::{get_file_reader, Answer};

pub fn solution() -> std::io::Result<Answer<u64>> {
    let reader = get_file_reader("input/aoc_2022/aoc_2022_7.txt")?;

    let root = Rc::new(Dir {
        name: "/".to_owned(),
        parent: Weak::new(),
        subdirs: RefCell::new(Vec::new()),
        size: RefCell::new(0),
    });

    let mut current_dir = root.clone();

    for line in reader.lines() {
        let line = line?;

        let input: Vec<&str> = line.split(' ').collect();
        if line.starts_with('$') {
            match *input.get(1).unwrap() {
                "ls" => (),
                "cd" => match *input.get(2).unwrap() {
                    "/" => current_dir = root.clone(),
                    ".." => {
                        if let Some(parent) = current_dir.parent.upgrade() {
                            current_dir = parent;
                        }
                    }
                    x => {
                        let mut temp = None;
                        for dir in current_dir.subdirs.borrow().iter() {
                            if dir.name == x {
                                temp = Some(dir.clone());
                                break;
                            }
                        }
                        if let Some(d) = temp {
                            current_dir = d;
                        }
                    }
                },
                _ => (),
            };
        } else if line.starts_with("dir") {
            let new_dir = Rc::new(Dir {
                name: (*input.get(1).unwrap()).to_owned(),
                parent: Rc::downgrade(&current_dir),
                subdirs: RefCell::new(Vec::new()),
                size: RefCell::new(0),
            });
            current_dir.subdirs.borrow_mut().push(new_dir);
        } else {
            let size: u64 = (*input.first().unwrap()).parse().unwrap();
            *current_dir.size.borrow_mut() += size;
        }
    }

    let mut max_100000 = Vec::new();
    update_sizes(&root, &mut max_100000);

    let needed_to_delete = *root.size.borrow() - (70000000 - 30000000);
    let mut part_2 = 70000000;
    find_smallest_to_delete(&root, needed_to_delete, &mut part_2);

    Ok(Answer {
        part_1: max_100000.iter().sum(),
        part_2,
    })
}

fn update_sizes(root: &Rc<Dir>, max_100000: &mut Vec<u64>) {
    for subdir in root.subdirs.borrow_mut().iter_mut() {
        update_sizes(subdir, max_100000);
        *root.size.borrow_mut() += *subdir.size.borrow();
    }
    if *root.size.borrow() < 100000 {
        max_100000.push(*root.size.borrow());
    }
}

fn find_smallest_to_delete(root: &Rc<Dir>, min: u64, current_smallest: &mut u64) -> u64 {
    for subdir in root.subdirs.borrow().iter() {
        let subdir_size = find_smallest_to_delete(subdir.borrow(), min, current_smallest);
        if subdir_size > min && subdir_size < *current_smallest {
            *current_smallest = subdir_size
        }
    }

    *root.size.borrow()
}

struct Dir {
    name: String,
    parent: Weak<Dir>,
    subdirs: RefCell<Vec<Rc<Dir>>>,
    size: RefCell<u64>,
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
                part_1: 1543140,
                part_2: 1117448
            }
        );
    }
}
