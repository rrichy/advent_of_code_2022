use std::time::Instant;

use crate::read_txt_file;

pub fn solve() {
    println!("DAY 10");

    part_one();
    part_two();
}

fn should_paint(register: i32, cycle: i32) -> bool {
    let position = cycle % 40;
    register - 1 == position || register == position || register + 1 == position
}

fn part_one() -> i32 {
    let start = Instant::now();
    let input = read_txt_file(10, crate::TextEnum::Input);

    let mut cycle: i32 = 0;
    let mut register: i32 = 1;
    let mut sum: i32 = 0;

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let command = line[0];
        let value: i32 = if line.len() == 2 {
            line[1].parse().unwrap_or(0)
        } else {
            0
        };

        let init_cycle = cycle;

        loop {
            cycle += 1;
            if (20..=220).step_by(40).collect::<Vec<_>>().contains(&cycle) {
                sum += cycle * register;
            }

            if command == "noop" || cycle - init_cycle == 2 {
                break;
            }
        }

        if command != "noop" {
            register += value
        }
    }

    println!("Sum of these six signal strength is: {}", sum);
    println!("Solved in: {:?}", start.elapsed());

    sum
}

fn part_two() -> Vec<String> {
    let start = Instant::now();
    let input = read_txt_file(10, crate::TextEnum::Input);

    let mut cycle = 0;
    let mut register = 1;
    let mut crt = ['.'; 240];

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let command = line[0];
        let value: i32 = if line.len() == 2 {
            line[1].parse().unwrap_or(0)
        } else {
            0
        };

        let init_cycle = cycle;

        loop {
            if should_paint(register, cycle) {
                crt[cycle as usize] = '#';
            }

            cycle += 1;
            if command == "noop" || cycle - init_cycle == 2 {
                break;
            }
        }

        if command != "noop" {
            register += value;
        }
    }

    let mut v = vec![];
    for i in (0..240).step_by(40) {
        let row: String = crt[i..(i + 40)].iter().collect();
        println!("{:?}", &row);
        v.push(row);
    }
    println!("Solved in: {:?}", start.elapsed());

    v
}

#[cfg(test)]
mod day_ten_tests {
    use super::*;

    #[test]
    fn part_one_should_be_correct() {
        assert_eq!(14620, part_one(), "Day 10 Part 1 should be 14620");
    }

    #[test]
    fn part_two_should_be_correct() {
        let answer = [
            "###....##.####.###..#..#.###..####.#..#.",
            "#..#....#.#....#..#.#..#.#..#.#....#..#.",
            "###.....#.###..#..#.####.#..#.###..#..#.",
            "#..#....#.#....###..#..#.###..#....#..#.",
            "#..#.#..#.#....#.#..#..#.#.#..#....#..#.",
            "###...##..#....#..#.#..#.#..#.#.....##..",
        ]
        .map(String::from)
        .to_vec();

        assert_eq!(answer, part_two(), "Day 10 Part 2 should be BJFRHRFU");
    }
}
