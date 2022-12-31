use std::time::Instant;

use crate::read_input;

pub fn solve() {
    println!("DAY 1");

    part_one();
    part_two();
}

pub fn main() -> (u32, u32, u32) {
    let input = read_input(1);
    let (mut first, mut second, mut third): (u32, u32, u32) = (0, 0, 0);

    for calories_per_elves in input.split("\r\n\r\n") {
        let mut total: u32 = 0;
        for calories in calories_per_elves.split("\r\n") {
            match calories.trim().parse::<u32>() {
                Ok(num) => total += num,
                Err(_) => panic!("Got a non-number calorie!"),
            };
        }

        if total > first {
            (first, second, third) = (total, first, second);
        } else if total > second {
            (first, second, third) = (first, total, second);
        } else if total > third {
            (first, second, third) = (first, second, total);
        }
    }

    (first, second, third)
}

pub fn part_one() -> u32 {
    let start = Instant::now();
    let (first, _s, _t) = main();

    println!(
        "The elf that is carrying the most calories has a total of: {} calories!",
        first
    );
    println!("Solved in: {:?}", start.elapsed());

    first
}

pub fn part_two() -> u32 {
    let start = Instant::now();
    let (first, second, third) = main();
    let sum = first + second + third;

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