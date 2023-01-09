use std::time::Instant;

use crate::read_txt_file;

pub fn solve() {
    println!("DAY 8");

    part_one();
    part_two();
}

fn part_one() -> u32 {
    let start = Instant::now();
    let input = read_txt_file(8, crate::TextEnum::Input);

    let rows = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visible: u32 = 0;
    for (y, row) in rows.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            // [left, top, right, bottom]
            let mut visibility = [true; 4];

            // check vertically, top to bottom
            for i in (0..y).rev() {
                if &rows[i][x] >= t {
                    visibility[1] = false;
                    break;
                }
            }

            // checking vertically, bottom to top
            for i in (y + 1)..rows.len() {
                if &rows[i][x] >= t {
                    visibility[3] = false;
                    break;
                }
            }

            // check horizontally, left to right
            for i in (0..x).rev() {
                if row[i] >= *t {
                    visibility[0] = false;
                    break;
                }
            }

            // check horizontally, right to left
            for i in (x + 1)..(row.len()) {
                if row[i] >= *t {
                    visibility[2] = false;
                    break;
                }
            }

            if visibility.contains(&true) {
                visible += 1;
            }
        }
    }
    println!("Number of visible trees: {:?}", visible);
    println!("Solved in: {:?}", start.elapsed());

    visible
}

fn part_two() -> u32 {
    let start = Instant::now();
    let input = read_txt_file(8, crate::TextEnum::Input);

    let rows = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut max_scenic = 0;
    for (y, row) in rows.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            // [left, top, right, bottom]
            let mut scenic = [0; 4];

            // check vertically, top to bottom
            for i in (0..y).rev() {
                scenic[1] += 1;
                if &rows[i][x] >= t {
                    break;
                }
            }

            // checking vertically, bottom to top
            for i in (y + 1)..rows.len() {
                scenic[3] += 1;
                if &rows[i][x] >= t {
                    break;
                }
            }

            // check horizontally, left to right
            for i in (0..x).rev() {
                scenic[0] += 1;
                if row[i] >= *t {
                    break;
                }
            }

            // check horizontally, right to left
            for i in (x + 1)..(row.len()) {
                scenic[2] += 1;
                if row[i] >= *t {
                    break;
                }
            }
            
            max_scenic = *[max_scenic, scenic.iter().product()].iter().max().unwrap();
        }
    }
    println!("Max scenic attainable: {:?}", max_scenic);
    println!("Solved in: {:?}", start.elapsed());

    max_scenic
}

#[cfg(test)]
mod day_eight_tests {
    use super::*;

    #[test]
    fn part_one_should_be_correct() {
        assert_eq!(1662, part_one(), "Day 8 Part 1 should be 1662");
    }

    #[test]
    fn part_two_should_be_correct() {
        assert_eq!(537600, part_two(), "Day 8 Part 2 should be 537600");
    }
}
