use itertools::Itertools;
use std::time::Instant;

use crate::read_txt_file;

pub fn solve() {
    println!("DAY 6");

    part_one();
    part_two();
}

fn part_one() -> usize {
    let start = Instant::now();
    let input = read_txt_file(6, crate::TextEnum::Input);

    let marker = input
        .as_bytes()
        .windows(4)
        .position(|packet| packet.iter().all_unique())
        .unwrap()
        + 4;

    println!("First marker detected after: {}", marker);
    println!("Solved in: {:?}", start.elapsed());

    marker
}

fn part_two() -> usize {
    let start = Instant::now();
    let input = read_txt_file(6, crate::TextEnum::Input);

    let message = input
        .as_bytes()
        .windows(14)
        .position(|packet| packet.iter().all_unique())
        .unwrap()
        + 14;

    println!("First message detected after: {}", message);
    println!("Solved in: {:?}", start.elapsed());

    message
}

#[cfg(test)]
mod day_six_tests {
    use super::*;

    #[test]
    fn part_one_should_be_correct() {
        assert_eq!(1034, part_one(), "Day 6 Part 1 should be 1034");
    }

    #[test]
    fn part_two_should_be_correct() {
        assert_eq!(2472, part_two(), "Day 6 Part 2 should be 2472");
    }
}
