use std::time::Instant;

use crate::read_input;

pub fn solve() {
    println!("DAY 4");
    
    part_one();
    part_two();
}

fn part_one() -> u32 {
    let start = Instant::now();
    let input = read_input(4);
    let mut total: u32 = 0;
    for range_pair in input.split("\r\n") {
        let vec: Vec<u32> = range_pair
            .split(&['-', ','][..])
            .map(|n| n.parse::<u32>().expect("Should be an integer"))
            .collect();

        let (mut min_1, mut max_1, mut min_2, mut max_2) = (vec[0], vec[1], vec[2], vec[3]);
        if min_1 > min_2 {
            (min_1, max_1, min_2, max_2) = (min_2, max_2, min_1, max_1);
        }

        if (min_1 >= min_2 && max_1 <= max_2) || (min_1 <= min_2 && max_1 >= max_2) {
            total += 1;
        }
    }
    println!(
        "Total assignment pairs that has one range fully contain the other is {}",
        total
    );
    println!("Solved in: {:?}", start.elapsed());

    total
}

fn part_two() -> u32 {
    let start = Instant::now();
    let input = read_input(4);
    let mut total: u32 = 0;
    for range_pair in input.split("\r\n") {
        let vec: Vec<u32> = range_pair
            .split(&['-', ','][..])
            .map(|n| n.parse::<u32>().expect("Should be an integer"))
            .collect();

        let (mut min_1, mut max_1, mut min_2, mut max_2) = (vec[0], vec[1], vec[2], vec[3]);
        if min_1 > min_2 {
            (min_1, max_1, min_2, max_2) = (min_2, max_2, min_1, max_1);
        }
        if (max_1 >= min_2)
            || (min_1 >= min_2 && max_1 <= max_2)
            || (min_1 <= min_2 && max_1 >= max_2)
        {
            total += 1;
        }
    }
    println!("Total assignment pairs that overlaps is {}", total);
    println!("Solved in: {:?}", start.elapsed());

    total
}

#[cfg(test)]
mod day_four_tests {
    use super::*;

    #[test]
    fn part_one_should_be_correct() {
        assert_eq!(576, part_one(), "Day 4 - Part 1 should be 576");
    }

    #[test]
    fn part_two_should_be_correct() {
        assert_eq!(905, part_two(), "Day 4 - Part 2 should be 905");
    }
}