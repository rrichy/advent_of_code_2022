use std::time::Instant;

use regex::Regex;

use crate::read_txt_file;

pub fn solve() {
    println!("DAY 5");

    part_one();
    part_two();
}

fn part_one() -> String {
    let start = Instant::now();
    let input = read_txt_file(5, crate::TextEnum::Input);

    let mut cargo: Vec<Vec<char>> = vec![];
    let (initial, moves) = input.split_once("\r\n\r\n").unwrap();
    for row in initial.lines().rev().skip(1) {
        for (index, c) in row.chars().skip(1).step_by(4).enumerate() {
            if c.is_alphabetic() {
                match cargo.get_mut(index) {
                    Some(v) => v.push(c),
                    None => cargo.insert(index, vec![c]),
                }
            }
        }
    }

    let re = Regex::new(r"\d+").unwrap();
    for line in moves.lines() {
        let (count, from, to) = match re
            .find_iter(line)
            .map(|d| d.as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()[..]
        {
            [a, b, c] => (a, b, c),
            _ => unreachable!(),
        };

        for _ in 0..count {
            let temp = cargo[from - 1].pop().unwrap();
            cargo[to - 1].push(temp);
        }
    }
    let top = cargo.iter().fold(String::new(), |mut s, c| {
        s.push(*c.last().unwrap());
        s
    });
    println!("The top crates are: {}", top);
    println!("Solved in: {:?}", start.elapsed());

    top
}

fn part_two() -> String {
    let start = Instant::now();
    let input = read_txt_file(5, crate::TextEnum::Input);

    let mut cargo: Vec<Vec<char>> = vec![];
    let (initial, moves) = input.split_once("\r\n\r\n").unwrap();
    for row in initial.lines().rev().skip(1) {
        for (index, c) in row.chars().skip(1).step_by(4).enumerate() {
            if c.is_alphabetic() {
                match cargo.get_mut(index) {
                    Some(v) => v.push(c),
                    None => cargo.insert(index, vec![c]),
                }
            }
        }
    }

    let re = Regex::new(r"\d+").unwrap();
    for line in moves.lines() {
        let (count, from, to) = match re
            .find_iter(line)
            .map(|d| d.as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()[..]
        {
            [a, b, c] => (a, b, c),
            _ => unreachable!(),
        };

        let start = cargo[from - 1].len() - count;
        let mut temp: Vec<char> = cargo[from - 1].splice(start.., []).collect();
        cargo[to - 1].append(&mut temp);
    }
    let top = cargo.iter().fold(String::new(), |mut s, c| {
        s.push(*c.last().unwrap());
        s
    });
    println!("The top crates with CrateMover 9001 are: {}", top);
    println!("Solved in: {:?}", start.elapsed());

    top
}

#[cfg(test)]
mod day_five_tests {
    use super::*;

    #[test]
    fn part_one_should_be_correct() {
        assert_eq!(
            "FCVRLMVQP".to_string(),
            part_one(),
            "Day 5 - Part 1 should be FCVRLMVQP"
        );
    }

    #[test]
    fn part_two_should_be_correct() {
        assert_eq!(
            "RWLWGJGFD".to_string(),
            part_two(),
            "Day 5 - Part 2 should be RWLWGJGFD"
        );
    }
}
