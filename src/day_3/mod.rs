use std::{collections::HashSet, time::Instant};

use crate::read_input;

pub fn solve() {
    println!("DAY 3");

    part_one();
    part_two();
}

pub fn part_one() -> u32 {
    let start = Instant::now();
    let input = read_input(3);

    let total = input
        .lines()
        .map(|r| {
            let (left, right) = r.split_at(r.len() / 2);
            for c in HashSet::<char>::from_iter(left.chars()) {
                if right.contains(c) {
                    let d = c as u32;
                    return match c.is_ascii_lowercase() {
                        true => d - 96,
                        false => d - 64 + 26,
                    };
                }
            }
            0
        })
        .sum();
    println!("Sum: {}, solved in: {:?}", total, start.elapsed());

    total
}

fn part_two() -> u32 {
    let start = Instant::now();
    let input = read_input(3);

    let total: u32 = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            for c in HashSet::<char>::from_iter(chunk[0].chars()) {
                if chunk[1].contains(c) && chunk[2].contains(c) {
                    let d = c as u32;
                    return match c.is_ascii_lowercase() {
                        true => d - 96,
                        false => d - 64 + 26,
                    };
                }
            }
            0
        })
        .sum();
    println!("Sum: {}, solved in: {:?}", total, start.elapsed());

    total
}

#[cfg(test)]
mod day_three_tests {
    use super::*;

    #[test]
    fn part_one_should_be_correct() {
        assert_eq!(7917, part_one(), "Day 3 - Part 1 should be 7917");
    }

    #[test]
    fn part_two_should_be_correct() {
        assert_eq!(2585, part_two(), "Day 3 - Part 2 should be 2585");
    }
}
