use std::{cmp::Ordering, collections::HashSet, time::Instant};

use crate::read_txt_file;

struct Point {
    x: i32,
    y: i32,
    visited: HashSet<String>,
    tail: Option<Box<Point>>,
}

impl Point {
    fn new_head(length: u32) -> Self {
        let mut head = Point::new(0, 0);

        let mut current_point = &mut head;
        for _ in 1..length {
            current_point.tail = Some(Box::new(Point::new(0, 0)));
            current_point = current_point.tail.as_mut().unwrap();
        }

        head
    }

    fn new(x: i32, y: i32) -> Self {
        Point {
            x,
            y,
            visited: HashSet::from([String::from("0,0")]),
            tail: None,
        }
    }

    fn is_touching(&self, p: &Point) -> bool {
        let delta_x = p.x - &self.x;
        let delta_y = p.y - &self.y;

        delta_x.abs() <= 1 && delta_y.abs() <= 1
    }

    fn is_tail_touching(&self) -> bool {
        match &self.tail {
            None => true,
            Some(p) => p.is_touching(self),
        }
    }

    fn move_to(&mut self, p: &Point) {
        self.x += match p.x.cmp(&self.x) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };
        self.y += match p.y.cmp(&self.y) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };

        self.visited
            .insert(String::from(format!("{},{}", self.x, self.y)));

        if !self.is_tail_touching() {
            let to_point = Point::new(self.x, self.y);
            self.tail.as_mut().unwrap().move_to(&to_point);
        }
    }
}

pub fn solve() {
    println!("DAY 9");

    part_one();
    part_two();
}

fn move_head(head: &mut Point) {
    let input = read_txt_file(9, crate::TextEnum::Input);

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let m: u32 = split[1].parse().expect("Expected to get an integer!");

        for _ in 0..m {
            let mut target = Point::new(head.x, head.y);
            if split[0] == "U" {
                target.y += 1;
            } else if split[0] == "D" {
                target.y -= 1;
            } else if split[0] == "L" {
                target.x -= 1;
            } else {
                target.x += 1;
            }

            head.move_to(&target);
        }
    }
}

fn part_one() -> u32 {
    let start = Instant::now();
    let mut head = Point::new_head(2);

    move_head(&mut head);
    let tail_visited_count: u32 = head.tail.unwrap().visited.len().try_into().unwrap();

    println!("{:?}", tail_visited_count);
    println!("Solved in: {:?}", start.elapsed());

    tail_visited_count
}

fn part_two() -> u32 {
    let start = Instant::now();
    let mut head = Point::new_head(10);

    move_head(&mut head);

    let mut current_point = &head;
    let tail_visited_count: u32 = loop {
        match &current_point.tail {
            None => break current_point.visited.len().try_into().unwrap(),
            Some(p) => current_point = p,
        }
    };

    println!("{:?}", tail_visited_count);
    println!("Solved in: {:?}", start.elapsed());

    tail_visited_count
}

#[cfg(test)]
mod day_nine_tests {
    use super::*;

    #[test]
    fn part_one_should_be_correct() {
        assert_eq!(6522, part_one(), "Day 9 Part 1 should be 6522");
    }

    #[test]
    fn part_two_should_be_correct() {
        assert_eq!(2717, part_two(), "Day 9 Part 2 should be 2717");
    }
}
