use crate::read_input;

pub fn solve() {
    println!("DAY 4");
    let input = read_input(4);
    
    let mut total: u32 = 0;
    for range_pair in input.split("\r\n") {
        let vec: Vec<u32> = range_pair.split(&['-', ','][..])
            .map(|n| n.parse::<u32>().expect("Should be an integer"))
            .collect();

        let (mut min_1, mut max_1, mut min_2, mut max_2) = (vec[0], vec[1], vec[2], vec[3]);
        // setting the left range to always be the least
        if min_1 > min_2 {
            (min_1, max_1, min_2, max_2) = (min_2, max_2, min_1, max_1);
        }

        if (min_1 >= min_2 && max_1 <= max_2) || (min_1 <= min_2 && max_1 >= max_2) {
            total += 1;
        }
    }
    
    println!("Total assignment pairs that has one range fully contain the other is {}", total);

    let mut total: u32 = 0;
    for range_pair in input.split("\r\n") {
        let vec: Vec<u32> = range_pair.split(&['-', ','][..])
            .map(|n| n.parse::<u32>().expect("Should be an integer"))
            .collect();

        let (mut min_1, mut max_1, mut min_2, mut max_2) = (vec[0], vec[1], vec[2], vec[3]);
        // setting the left range to always be the least
        if min_1 > min_2 {
            (min_1, max_1, min_2, max_2) = (min_2, max_2, min_1, max_1);
        }
        if (max_1 >= min_2) || (min_1 >= min_2 && max_1 <= max_2) || (min_1 <= min_2 && max_1 >= max_2) {
            total += 1;
        }
    }
    
    println!("Total assignment pairs that overlaps is {}", total);
}    