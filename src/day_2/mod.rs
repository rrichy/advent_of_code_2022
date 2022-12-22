use std::collections::HashMap;

use crate::read_input;

pub fn solve() {
    println!("DAY 2");
    let input = read_input(2);
    let mut total: u32 = 0;

    let mut map: HashMap<&str, u32> = HashMap::new();

    map.insert("A X", 1 + 3); // rock vs rock
    map.insert("A Y", 2 + 6); // rock vs paper
    map.insert("A Z", 3 + 0); // rock vs scissors
    map.insert("B X", 1 + 0); // paper vs rock
    map.insert("B Y", 2 + 3); // paper vs paper
    map.insert("B Z", 3 + 6); // paper vs rock
    map.insert("C X", 1 + 6); // scissors vs rock
    map.insert("C Y", 2 + 0); // scissors vs paper
    map.insert("C Z", 3 + 3); // scissors vs scissors

    for round in input.split("\r\n") {
        total += map.get(round).unwrap();
    }

    println!("Won Rock Paper Scissors with a score of {}!", total);

    let mut total: u32 = 0;

    let mut map: HashMap<&str, u32> = HashMap::new();

    map.insert("A X", 3 + 0); // rock vs scissors
    map.insert("A Y", 1 + 3); // rock vs rock
    map.insert("A Z", 2 + 6); // rock vs paper
    map.insert("B X", 1 + 0); // paper vs rock
    map.insert("B Y", 2 + 3); // paper vs paper
    map.insert("B Z", 3 + 6); // paper vs scissors
    map.insert("C X", 2 + 0); // scissors vs paper
    map.insert("C Y", 3 + 3); // scissors vs scissors
    map.insert("C Z", 1 + 6); // scissors vs rock

    for round in input.split("\r\n") {
        total += map.get(round).unwrap();
    }

    println!("Won Rock Paper Scissors with a score of {}!", total);
}
