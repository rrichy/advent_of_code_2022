use std::{collections::HashSet, time::Instant};

use crate::read_txt_file;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn is_touching(&self, p: &Point) -> bool {
        let delta_x = p.x - &self.x;
        let delta_y = p.y - &self.y;

        delta_x.abs() <= 1 && delta_y.abs() <= 1
    }

    fn move_to(&mut self, p: &Point) {
        self.x = p.x;
        self.y = p.y;
    }

    fn move_to_touch(&mut self, h: &Point, o: &Point) -> String {
        if !self.is_touching(h) {
            self.move_to(o);
        }

        String::from(format!("{},{}", &self.x, &self.y))
    }

    fn move_with_tail(&mut self, to: &Point, t: &mut Point) -> String {
        let o = Point::new(self.x, self.y);
        let _ = &self.move_to(to);
        let s = t.move_to_touch(self, &o);

        s
    }
}

pub fn solve() {
    println!("DAY 9");

    part_one();
    part_two();
    
    // let mut rope = Vec::<Point>::new();
    // let mut visited = HashSet::<String>::new();

    // for _ in 0..10 {
    //     rope.push(Point::new(0, 0));
    // }

    // for line in input.lines() {
    //     let split = line.split_whitespace().collect::<Vec<_>>();
    //     let m: u32 = split[1].parse().expect("Expected to get an integer!");

    //     for _ in 0..m {
    //         let head = rope.first().unwrap();
    //         let mut target = Point::new(head.x, head.y);
    //         if split[0] == "U" {
    //             target.y += 1;
    //         } else if split[0] == "D" {
    //             target.y -= 1;
    //         } else if split[0] == "L" {
    //             target.x -= 1;
    //         } else {
    //             target.x += 1;
    //         }

    //         // for tail in rope {
    //         //     head.move_with_tail(&target, &mut tail);
    //         //     head = &tail;

    //         // }

    //         // let vis = *head.move_with_tail(&target, &mut tail);
    //         // visited.insert(vis);
    //     }
    // }


}

fn part_one() -> u32 {
    let start = Instant::now();
    let input = read_txt_file(9, crate::TextEnum::Input);

    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    let mut visited = HashSet::<String>::new();

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

            let vis = head.move_with_tail(&target, &mut tail);
            visited.insert(vis);
        }
    }
    println!("{:?}", visited.len());
    println!("Solved in: {:?}", start.elapsed());

    visited.len() as u32
}

fn part_two() {
    let input = read_txt_file(9, crate::TextEnum::Input);

}

#[cfg(test)]
mod day_nine_tests {
    use super::*;

    #[test]
    fn part_one_should_be_correct() {
        assert_eq!(6522, part_one(), "Day 9 Part 1 should be 6522");
    }

    // #[test]
    // fn part_two_should_be_correct() {
    //     assert_eq!(537600, part_two(), "Day 8 Part 2 should be 537600");
    // }
}