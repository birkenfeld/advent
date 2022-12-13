use advtools::input;
use advtools::prelude::Itertools;
use std::cmp::Ordering;

#[derive(PartialEq, Clone)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

fn compare(p1: &Packet, p2: &Packet) -> Ordering {
    match (p1, p2) {
        (Packet::Int(i1), Packet::Int(i2)) => i1.cmp(i2),
        (l1 @ Packet::List(_), Packet::Int(i2)) => {
            compare(l1, &Packet::List(vec![Packet::Int(*i2)]))
        }
        (Packet::Int(i1), l2 @ Packet::List(_)) => {
            compare(&Packet::List(vec![Packet::Int(*i1)]), l2)
        }
        (Packet::List(l1), Packet::List(l2)) => {
            for (c1, c2) in l1.iter().zip(l2) {
                match compare(c1, c2) {
                    Ordering::Equal => (),
                    result => return result
                }
            }
            l1.len().cmp(&l2.len())
        }
    }
}

fn parse(input: &mut dyn Iterator<Item=&u8>) -> Packet {
    let mut input = input.peekable();
    let mut items = Vec::new();
    while let Some(ch) = input.next() {
        match ch {
            b'[' => items.push(parse(&mut input)),
            b']' => break,
            b',' => continue,
            dig => {
                let mut num = (dig - b'0') as i32;
                while input.peek().map_or(false, |v| v.is_ascii_digit()) {
                    num = 10*num + (input.next().unwrap() - b'0') as i32;
                }
                items.push(Packet::Int(num));
            }
        }
    }
    Packet::List(items)
}

fn main() {
    let mut packets = input::lines().map(
        |line| parse(&mut line.as_bytes().iter().skip(1))
    ).collect_vec();

    let ordered_index_sum = packets.iter().tuples().enumerate()
        .filter_map(|(i, (p1, p2))| (compare(p1, p2) == Ordering::Less).then_some(i + 1))
        .sum::<usize>();
    advtools::verify("Sum of correct indices", ordered_index_sum, 5529);

    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort_by(compare);
    let decoder_key = packets.iter().enumerate()
        .filter_map(|(i, p)| (p == &div1 || p == &div2).then_some(i + 1))
        .product::<usize>();
    advtools::verify("Decoder key", decoder_key, 27690);
}
