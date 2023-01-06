mod mod_alt;

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

    let mut dir_stack = Vec::<&str>::new();
    let mut dir_sizes = HashMap::<String, u32>::new();
    for line in input.lines() {
        if line.starts_with("$") {
            let dir = line.split_whitespace().last().unwrap();
            if dir != "ls" {
                // on cd xxxxx
                if dir != ".." {
                    dir_stack.push(&dir);

                    let dir = dir_stack.to_vec().join("/");
                    dir_sizes.insert(dir, 0);
                // on cd ..
                } else {
                    dir_stack.pop();
                }
            }
        } else {
            let mut words = line.split_whitespace();
            let first = words.next();

            // when line starts with size
            if first != Some("dir") {
                let size = first
                    .unwrap()
                    .parse::<u32>()
                    .expect("Expected to have integer!");

                let mut directories = Vec::<String>::new();

                for folder in &dir_stack {
                    match directories.last() {
                        Some(prev) => directories.push(format!("{}/{}", prev, folder)),
                        None => directories.push(String::from(*folder)),
                    }
                }

                for dir in directories {
                    dir_sizes.entry(dir).and_modify(|s| *s += size).or_insert(0);
                }
            }
        }
    }
    let total: u32 = dir_sizes.values().filter(|v| v <= &&(100000 as u32)).sum();
    println!("Sum of directories: {}", total);
    println!("Solved in: {:?}", start.elapsed());

    total
}

fn part_two() -> u32 {
    let start = Instant::now();
    let input = read_txt_file(7, crate::TextEnum::Input);

    let mut dir_stack = Vec::<&str>::new();
    let mut dir_sizes = HashMap::<String, u32>::new();

    for line in input.lines() {
        if line.starts_with("$") {
            let dir = line.split_whitespace().last().unwrap();
            if dir != "ls" {
                // on cd xxxxx
                if dir != ".." {
                    dir_stack.push(&dir);

                    let dir = dir_stack.to_vec().join("/");
                    dir_sizes.insert(dir, 0);
                // on cd ..
                } else {
                    dir_stack.pop();
                }
            }
        } else {
            let mut words = line.split_whitespace();
            let first = words.next();

            // when line starts with size
            if first != Some("dir") {
                let size = first
                    .unwrap()
                    .parse::<u32>()
                    .expect("Expected to have integer!");

                let mut directories = Vec::<String>::new();

                for folder in &dir_stack {
                    match directories.last() {
                        Some(prev) => directories.push(format!("{}/{}", prev, folder)),
                        None => directories.push(String::from(*folder)),
                    }
                }

                for dir in directories {
                    dir_sizes.entry(dir).and_modify(|s| *s += size).or_insert(0);
                }
            }
        }
    }

    let ununsed_space = 70_000_000 - dir_sizes.get("/").unwrap();
    let required_space = 30_000_000 - &ununsed_space;
    let least = *dir_sizes
        .values()
        .filter(|v| v > &&required_space)
        .min()
        .unwrap();

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
