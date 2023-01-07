use std::time::Instant;

use crate::read_txt_file;

pub fn solve() {
    println!("DAY 1");

    part_one();
    part_two();
}

fn part_one() -> u32 {
    let start = Instant::now();
    let input = read_txt_file(1, crate::TextEnum::Input);

    let max = input
        .split("\r\n\r\n")
        .map(|bag| {
            bag.split_ascii_whitespace()
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    println!(
        "The elf that is carrying the most calories has a total of: {} calories!",
        max
    );
    println!("Solved in: {:?}", start.elapsed());

    max
}

fn part_two() -> u32 {
    let start = Instant::now();
    let input = read_txt_file(1, crate::TextEnum::Input);

    let mut calories = input
        .split("\r\n\r\n")
        .map(|bag| {
            bag.split_ascii_whitespace()
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    calories.sort_unstable_by(|a, b| b.cmp(a));
    let sum = calories.iter().take(3).sum();

    println!(
        "The top three elves carrying the most calories has a total of: {} calories combined!",
        &sum
    );
    println!("Solved in: {:?}", start.elapsed());

    sum
}

#[cfg(test)]
mod day_one_tests {
    use super::*;

    #[test]
    fn part_one_should_be_correct() {
        assert_eq!(71471, part_one(), "Day 1 - Part 1 should be 71471");
    }

    #[test]
    fn part_two_should_be_correct() {
        assert_eq!(211189, part_two(), "Day 1 - Part 2 should be 211189");
    }
}
