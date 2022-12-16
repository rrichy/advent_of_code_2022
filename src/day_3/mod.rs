use std::collections::HashSet;

use crate::read_input;

pub fn solve() {
    println!("DAY 3");
    let input = read_input(3);
    let mut total: u32 = 0;

    for rucksacks in input.split("\r\n") {
        let len = &rucksacks.len() / 2;
        let set: HashSet<char> = HashSet::from_iter(&mut rucksacks[..len].chars());
        for c in rucksacks[len..].chars() {
            if set.contains(&c) {
                if c.is_ascii_lowercase() {
                    total += c as u32 - 96;
                } else {
                    total += c as u32 - 64 + 26;
                }
                break;
            }
        }
    }

    println!("Sum: {}", total);
    // 5168 too low
    // 10217 too high

    let mut total: u32 = 0;
    let mut chunks = input.split("\r\n");

    let mut counter = 0;
    let mut set: HashSet<char> = HashSet::new();
    loop {
        match chunks.next() {
            Some(rucksack) => {
                if counter == 0 {
                    set = HashSet::from_iter(rucksack.chars());
                } else {
                    let set2 = HashSet::from_iter(rucksack.chars());
                    set = HashSet::from_iter(&mut set.intersection(&set2).into_iter());
                    if counter == 2 {
                        for c in set.into_iter() {
                            if c.is_ascii_lowercase() {
                                total += c as u32 - 96;
                            } else {
                                total += c as u32 - 64 + 26;
                            }
                            break;
                        }
                        counter = 0;
                    }
                }
                counter += 1;
            },
            None => { break }
        }
    }

    println!("Sum priorities: {}", total);
}    