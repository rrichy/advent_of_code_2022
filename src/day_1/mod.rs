use crate::read_input;

pub fn solve() {
    println!("DAY 1");
    let (first, second, third) = main();

    part_one(&first);
    part_two(first, second, third);
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

pub fn part_one(first: &u32) -> &u32 {
    println!(
        "The elf that is carrying the most calories has a total of: {} calories!",
        first
    );

    first
}

pub fn part_two(first: u32, second: u32, third: u32) -> u32 {
    let sum = first + second + third;

    println!(
        "The top three elves carrying the most calories has a total of: {} calories combined!",
        &sum
    );

    sum
}

#[cfg(test)]
mod day_one_tests {
    use super::*;
        
    #[test]
    fn part_one_should_be_correct() {
        let (first, _s, _t) = main();
        assert_eq!(&71471, part_one(&first), "Day 1 - Part 1 should be 71471");
    }

    #[test]
    fn part_two_should_be_correct() {
        let (first, second, third) = main();
        assert_eq!(211189, part_two(first, second, third), "Day 1 - Part 2 should be 211189");
    }
}