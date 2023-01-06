use std::time::Instant;

use crate::read_input;

pub fn solve() {
    println!("DAY 6");

    part_one();
    part_two();
}

fn part_one() -> usize {
    let start = Instant::now();
    let input = read_input(6);
    let mut packet = Vec::<char>::from_iter(input.split_at(4).0.chars());
    let mut first = 0;
    for (marker, c) in input.char_indices() {
        let mut is_unique = true;

        packet.splice(..1, []);
        packet.push(c);

        for i in 0..4 {
            let mut sub = packet.clone();
            sub.remove(i);
            if sub.contains(&packet.get(i).unwrap()) {
                is_unique = false;
                break;
            }
        }

        if is_unique {
            first = marker + 1;
            break;
        }
    }
    println!("First marker detected after: {}", first);
    println!("Solved in: {:?}", start.elapsed());

    first
}

fn part_two() -> usize {
    let start = Instant::now();
    let input = read_input(6);
    let mut message = Vec::<char>::from_iter(input.split_at(14).0.chars());
    let mut first = 0;
    for (marker, c) in input.char_indices() {
        let mut is_unique = true;

        message.splice(..1, []);
        message.push(c);

        for i in 0..14 {
            let mut sub = message.clone();
            sub.remove(i);
            if sub.contains(&message.get(i).unwrap()) {
                is_unique = false;
                break;
            }
        }

        if is_unique {
            first = marker + 1;
            break;
        }
    }
    println!("First message detected after: {}", first);
    println!("Solved in: {:?}", start.elapsed());

    first
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
