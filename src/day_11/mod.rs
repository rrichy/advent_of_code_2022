use regex::Regex;

use crate::read_txt_file;

pub fn solve() {
    println!("DAY 11");
    let input = read_txt_file(11, crate::TextEnum::Input);

    let mut monkeys = get_monkeys(input);
    let modulus: u32 = monkeys.iter().map(|m| m.modulus).product();

    // for _ in 0..20 {
    //     proceed(&mut monkeys, &modulus);
    // }

    // let mut levels = monkeys.iter().map(|m| m.inspect).collect::<Vec<_>>();
    // levels.sort_by(|a, b| b.cmp(a));

    // println!("The level of monkey business: {}", levels[0] * levels[1]);

    for _ in 0..10_000 {
        proceed(&mut monkeys, &modulus);
    }

    let mut levels = monkeys.iter().map(|m| m.inspect).collect::<Vec<_>>();
    levels.sort_by(|a, b| b.cmp(a));

    println!("{}", (levels[0] as u64) * (levels[1] as u64));
}

fn proceed(monkeys: &mut Vec<Monkey>, modulus: &u32) {
    for i in 0..monkeys.len() {
        let pass = monkeys[i].play(modulus);

        for (j, worry) in pass {
            monkeys[j].add(worry);
        }
    }
}

fn get_monkeys(input: String) -> Vec<Monkey> {
    let re =
        Regex::new(r"^.+\r\n.+: (.*)\r\n.+:.+(\*|\+|-) (.+)\r\n.+ (\d+)\r\n.+ (\d+)\r\n.+ (\d+)$")
            .unwrap();
    let mut monkeys: Vec<Monkey> = Vec::new();

    for details in input.split("\r\n\r\n") {
        let caps = re.captures(details).unwrap();

        let items: Vec<u64> = caps
            .get(1)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        let operation = caps.get(2).unwrap().as_str().chars().next().unwrap();
        let number = match caps.get(3).unwrap().as_str() {
            "old" => Number::Old,
            n => Number::Constant(n.parse::<u64>().unwrap()),
        };
        let modulus: u32 = caps.get(4).unwrap().as_str().parse().unwrap();
        let if_true: usize = caps.get(5).unwrap().as_str().parse().unwrap();
        let if_false: usize = caps.get(6).unwrap().as_str().parse().unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            number,
            modulus,
            if_true,
            if_false,
            inspect: 0,
        });
    }

    monkeys
}

enum Number {
    Old,
    Constant(u64),
}

struct Monkey {
    items: Vec<u64>,
    operation: char,
    number: Number,
    modulus: u32,
    if_true: usize,
    if_false: usize,
    inspect: u32,
}

impl Monkey {
    fn play(&mut self, modulus: &u32) -> Vec<(usize, u64)> {
        let mut pass: Vec<(usize, u64)> = Vec::new();
        for item in &self.items {
            self.inspect += 1;
            let worry: u64 = match self.number {
                Number::Old => match self.operation {
                    '+' => item + item,
                    '-' => 0,
                    _ => item * item,
                },
                Number::Constant(n) => match self.operation {
                    '+' => item + n,
                    '-' => item - n,
                    _ => item * n,
                },
            } % (*modulus as u64);

            match worry % (self.modulus as u64) == 0 {
                true => {
                    pass.push((self.if_true.clone(), worry));
                }
                false => {
                    pass.push((self.if_false.clone(), worry));
                }
            }
        }

        self.items = vec![];

        pass
    }

    fn add(&mut self, item: u64) {
        self.items.push(item);
    }
}
