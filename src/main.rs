use std::io;

use advent_of_code_2022::{
    day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, day_9, structures::Day,
};

fn main() {
    println!("Advent of Code 2022");

    const SPECIFIC: u32 = 0;

    let day: Day = if SPECIFIC == 0 {
        loop {
            println!("Enter which day to solve:");

            let mut day = String::new();

            io::stdin()
                .read_line(&mut day)
                .expect("Failed to read line");

            match day.trim().parse::<u32>() {
                Ok(num) => break Day::new(num),
                Err(_) => continue,
            }
        }
    } else {
        Day::new(SPECIFIC)
    };

    match day.value() {
        1 => day_1::solve(),
        2 => day_2::solve(),
        3 => day_3::solve(),
        4 => day_4::solve(),
        5 => day_5::solve(),
        6 => day_6::solve(),
        7 => day_7::solve(),
        8 => day_8::solve(),
        9 => day_9::solve(),
        _ => panic!("Day {} has not yet been solved.", day.value()),
    }
}
