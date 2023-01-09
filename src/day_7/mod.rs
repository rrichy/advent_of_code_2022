use std::{collections::HashMap, time::Instant};

use crate::read_txt_file;

pub fn solve() {
    println!("DAY 7");

    part_one();
    part_two();
}

fn part_one() -> u32 {
    let start = Instant::now();
    let input = read_txt_file(7, crate::TextEnum::Input);

    let mut dir_stack = vec![];
    let mut dir_sizes = HashMap::new();

    for line in input.lines() {
        let line = line.split_whitespace().collect::<Vec<_>>();

        if let [_, _, name] = line[..] {
            if name == ".." {
                dir_stack.pop();
            } else {
                dir_stack.push(name);
            }
        }

        if let [a, _] = line[..] {
            if a != "$" && a != "dir" {
                let size: u32 = a.parse().unwrap();
                dir_stack.iter().fold(String::new(), |mut path, dir| {
                    path.push_str(format!("__{dir}").as_str());
                    dir_sizes
                        .entry(path.clone())
                        .and_modify(|s| *s += size)
                        .or_insert(size);

                    path
                });
            }
        }
    }

    let total = dir_sizes
        .values()
        .fold(0, |sum, v| if v <= &100000 { sum + v } else { sum });
    println!("Sum of directories: {}", total);
    println!("Solved in: {:?}", start.elapsed());

    total
}

fn part_two() -> u32 {
    let start = Instant::now();
    let input = read_txt_file(7, crate::TextEnum::Input);

    let mut dir_stack = vec![];
    let mut dir_sizes = HashMap::new();

    for line in input.lines() {
        let line = line.split_whitespace().collect::<Vec<_>>();

        if let [_, _, name] = line[..] {
            if name == ".." {
                dir_stack.pop();
            } else {
                dir_stack.push(name);
            }
        }

        if let [a, _] = line[..] {
            if a != "$" && a != "dir" {
                let size: u32 = a.parse().unwrap();
                dir_stack.iter().fold(String::new(), |mut path, dir| {
                    path.push_str(format!("__{dir}").as_str());
                    dir_sizes
                        .entry(path.clone())
                        .and_modify(|s| *s += size)
                        .or_insert(size);

                    path
                });
            }
        }
    }

    let ununsed_space = 70_000_000 - dir_sizes.get("__/").unwrap();
    let required_space = 30_000_000 - &ununsed_space;
    let least = dir_sizes.values().fold(70_000_000, |min, v| {
        if v > &required_space && v < &min {
            *v
        } else {
            min
        }
    });

    println!("Least size directory: {}", least);
    println!("Solved in: {:?}", start.elapsed());

    least
}

#[cfg(test)]
mod day_seven_tests {
    use super::*;

    #[test]
    fn part_one_should_be_correct() {
        assert_eq!(1118405, part_one(), "Day 7 Part 1 should be 1118405");
    }

    #[test]
    fn part_two_should_be_correct() {
        assert_eq!(12545514, part_two(), "Day 6 Part 2 should be 12545514");
    }
}
