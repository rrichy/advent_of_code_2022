use crate::read_input;

pub fn solve() {
    println!("DAY 6");
    let input = read_input(6);

    let mut packet = Vec::<char>::from_iter(input.split_at(4).0.chars());
    let mut first = 0;

    for (marker, c) in input.char_indices() {
        let mut is_unique = true;

        packet.splice(..1, []);
        packet.push(c);

        for i in 0..4 {
            let mut sub = packet.clone();
            sub.remove(i);
            if sub.contains(&packet.get(i).unwrap()) {
                is_unique = false;
                break;
            }
        }

        if is_unique {
            first = marker + 1;
            break;
        }
    }

    println!("First marker detected after: {}", first);

    let mut message = Vec::<char>::from_iter(input.split_at(14).0.chars());
    let mut first = 0;

    for (marker, c) in input.char_indices() {
        let mut is_unique = true;

        message.splice(..1, []);
        message.push(c);

        for i in 0..14 {
            let mut sub = message.clone();
            sub.remove(i);
            if sub.contains(&message.get(i).unwrap()) {
                is_unique = false;
                break;
            }
        }

        if is_unique {
            first = marker + 1;
            break;
        }
    }

    println!("First message detected after: {}", first);
}
