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
    let mut total: u32 = 0;

    for rucksacks in input.split("\r\n") {
        let len = &rucksacks.len() / 2;
        let set: HashSet<char> = HashSet::from_iter(&mut rucksacks[..len].chars());
        for c in rucksacks[len..].chars() {
            if set.contains(&c) {
                if c.is_ascii_lowercase() {
                    total += c as u32 - 96;
                } else {
                    total += c as u32 - 64 + 26;
                }
                break;
            }
        }
    }
    println!("Sum: {}, solved in: {:?}", total, start.elapsed());

    total
}

fn part_two() -> u32 {
    let start = Instant::now();
    let input = read_input(3);
    let mut total: u32 = 0;
    let mut rucksacks = input.split("\r\n");

    loop {
        let mut should_break = false;

        let mut set = HashSet::new();
        for _ in 0..3 {
            match rucksacks.next() {
                Some(rucksack) => {
                    if set.capacity() == 0 {
                        set = HashSet::from_iter(rucksack.chars());
                    } else {
                        let set2 = HashSet::from_iter(rucksack.chars());
                        let set3 = set.clone();

                        set.clear();
                        for c in set3.intersection(&set2) {
                            set.insert(*c);
                        }
                    }
                }
                None => {
                    should_break = true;
                    break;
                }
            }
        }

        for c in set.into_iter() {
            if c.is_ascii_lowercase() {
                total += c as u32 - 96;
            } else {
                total += c as u32 - 64 + 26;
            }
            break;
        }

        if should_break == true {
            break;
        }
    }
    println!(
        "Sum priorities: {}, solved in: {:?}",
        total,
        start.elapsed()
    );

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