use crate::read_input;

pub fn solve() {
    println!("DAY 1");
    let input = read_input(1);
    let (mut first, mut second, mut third): (u32, u32, u32) = (0, 0 ,0);

    for calories_per_elves in input.split("\r\n\r\n") {
        let mut total: u32 = 0;
        for calories in calories_per_elves.split("\r\n") {
            match calories.trim().parse::<u32>() {
                Ok(num) => total += num,
                Err(_) => panic!("Got a non-number calorie!"),
            };
        };

        if total > first {
            (first, second, third) = (total, first, second);
        } else if total > second {
            (first, second, third) = (first, total, second);
        } else if total > third {
            (first, second, third) = (first, second, total);
        }
    }

    println!("The elf that is carrying the most calories has a total of: {} calories!", first);
    println!("The top three elves carrying the most calories has a total of: {} calories combined!", first + second + third);
}    