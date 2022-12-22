use std::collections::HashMap;

use crate::read_input;

pub fn solve() {
    println!("DAY 5");
    let input = read_input(5);
    let mut cargo: HashMap<u32, Vec<char>> = HashMap::new();

    let split: Vec<&str> = input.split("\r\n\r\n").collect();
    let initial: Vec<&str> = split[0].lines().collect();
    let stack_line = initial[initial.len() - 1];

    for (i, stack_number) in stack_line.char_indices() {
        if stack_number.is_alphanumeric() {
            let key = stack_number.to_digit(10).unwrap();
            cargo.insert(key, Vec::new());

            for line in (0..(initial.len() - 1)).rev() {
                let c = initial[line].chars().nth(i).unwrap();
                if c.is_alphabetic() {
                    cargo.entry(key).and_modify(|stack| stack.push(c));
                }
            }
        }
    }

    for line in split[1].lines() {
        let mut vector = Vec::<u32>::new();

        for word in line.split_whitespace() {
            match word.parse::<u32>() {
                Ok(num) => vector.push(num),
                Err(_) => continue,
            }
        }

        let (count, from, to) = (vector[0], vector[1], vector[2]);

        let mut from_stack = cargo.get(&from).unwrap().clone();
        let mut to_stack = cargo.get(&to).unwrap().clone();

        for _ in 0..count {
            to_stack.push(from_stack.pop().unwrap());
        }

        cargo.insert(from, from_stack);
        cargo.insert(to, to_stack);
    }

    let mut top = String::new();
    for i in 1..=cargo.keys().len() {
        top.push(*cargo.get(&(i as u32)).unwrap().last().unwrap());
    }

    println!("The top crates are: {}", top);

    // PART 2
    let mut cargo: HashMap<u32, Vec<char>> = HashMap::new();

    let split: Vec<&str> = input.split("\r\n\r\n").collect();
    let initial: Vec<&str> = split[0].lines().collect();
    let stack_line = initial[initial.len() - 1];

    for (i, stack_number) in stack_line.char_indices() {
        if stack_number.is_alphanumeric() {
            let key = stack_number.to_digit(10).unwrap();
            cargo.insert(key, Vec::new());

            for line in (0..(initial.len() - 1)).rev() {
                let c = initial[line].chars().nth(i).unwrap();
                if c.is_alphabetic() {
                    cargo.entry(key).and_modify(|stack| stack.push(c));
                }
            }
        }
    }

    for line in split[1].lines() {
        let mut vector = Vec::<u32>::new();

        for word in line.split_whitespace() {
            match word.parse::<u32>() {
                Ok(num) => vector.push(num),
                Err(_) => continue,
            }
        }

        let (count, from, to) = (vector[0], vector[1], vector[2]);

        let mut from_stack = cargo.get(&from).unwrap().clone();
        let mut to_stack = cargo.get(&to).unwrap().clone();

        let exclusive_i = from_stack.len() - (count as usize);
        let mut end: Vec<char> = Vec::from(&from_stack[exclusive_i..]);

        from_stack.splice(exclusive_i.., []);
        to_stack.append(&mut end);

        cargo.insert(from, from_stack);
        cargo.insert(to, to_stack);
    }

    let mut top = String::new();
    for i in 1..=cargo.keys().len() {
        top.push(*cargo.get(&(i as u32)).unwrap().last().unwrap());
    }

    println!("The top crates with CrateMover 9001 are: {}", top);
}
