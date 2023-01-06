use std::{collections::HashMap, time::Instant};

use crate::read_input;

pub fn solve() {
    println!("DAY 2");

    part_one();
    part_two();
}

pub fn part_one() -> u32 {
    let start = Instant::now();
    let input = read_input(2);

    let hand_map = HashMap::from([
        ("A X", 1 + 3), // rock vs rock
        ("A Y", 2 + 6), // rock vs paper
        ("A Z", 3 + 0), // rock vs scissors
        ("B X", 1 + 0), // paper vs rock
        ("B Y", 2 + 3), // paper vs paper
        ("B Z", 3 + 6), // paper vs rock
        ("C X", 1 + 6), // scissors vs rock
        ("C Y", 2 + 0), // scissors vs paper
        ("C Z", 3 + 3), // scissors vs scissors
    ]);

    let total = input
        .lines()
        .map(|round| hand_map.get(round).unwrap_or(&0))
        .sum();

    println!("Won Rock Paper Scissors with a score of {}!", total);
    println!("Solved in: {:?}", start.elapsed());

    total
}

pub fn part_two() -> u32 {
    let start = Instant::now();
    let input = read_input(2);

    let hand_map = HashMap::from([
        ("A X", 3 + 0), // rock vs scissors
        ("A Y", 1 + 3), // rock vs rock
        ("A Z", 2 + 6), // rock vs paper
        ("B X", 1 + 0), // paper vs rock
        ("B Y", 2 + 3), // paper vs paper
        ("B Z", 3 + 6), // paper vs scissors
        ("C X", 2 + 0), // scissors vs paper
        ("C Y", 3 + 3), // scissors vs scissors
        ("C Z", 1 + 6), // scissors vs rock
    ]);

    let total = input
        .lines()
        .map(|round| hand_map.get(round).unwrap_or(&0))
        .sum();

    println!("Won Rock Paper Scissors with a score of {}!", total);
    println!("Solved in: {:?}", start.elapsed());

    total
}

#[cfg(test)]
mod day_two_tests {
    use super::*;

    #[test]
    fn part_one_should_be_correct() {
        assert_eq!(13446, part_one(), "Day 2 - Part 1 should be 13446");
    }

    #[test]
    fn part_two_should_be_correct() {
        assert_eq!(13509, part_two(), "Day 2 - Part 2 should be 13509");
    }
}
