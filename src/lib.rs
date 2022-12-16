use std::{env::current_dir, fs};

pub mod structures {
    pub struct Day {
        value: u32
    }

    impl Day {
        pub fn new(value: u32) -> Day {
            if value < 1 || value > 31 {
                panic!("Input a valid day. There are only 31 days in December!");
            }

            Day { value }
        }

        pub fn value(&self) -> u32 {
            self.value
        }
    }
}

pub fn read_input(day: u32) -> String {
    let cwd = current_dir()
        .expect("Failed to get the current working directory");

    fs::read_to_string(cwd.join(format!("src/day_{}/input.txt", day)))
        .expect("input.txt does not exists!")
}

pub fn read_sample(day: u32) -> String {
    let cwd = current_dir()
        .expect("Failed to get the current working directory");

    fs::read_to_string(cwd.join(format!("src/day_{}/sample.txt", day)))
        .expect("sample.txt does not exists!")
}

pub mod day_1;
pub mod day_2;
pub mod day_3;