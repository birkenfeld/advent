use advtools::input;
use advtools::prelude::Itertools;
use std::cmp::Ordering;
use self::Packet::*;

#[derive(serde::Deserialize, PartialEq, Clone)]
#[serde(untagged)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

fn compare(p1: &Packet, p2: &Packet) -> Ordering {
    match (p1, p2) {
        (Int(i1), Int(i2)) => i1.cmp(i2),
        (l1 @ List(_), Int(i2)) => compare(l1, &List(vec![Int(*i2)])),
        (Int(i1), l2 @ List(_)) => compare(&List(vec![Int(*i1)]), l2),
        (List(l1), List(l2)) => l1.iter().zip(l2)
                                         .map(|(c1, c2)| compare(c1, c2))
                                         .find(|&ord| ord.is_ne())
                                         .unwrap_or(l1.len().cmp(&l2.len()))
    }
}

fn main() {
    let mut packets = input::lines().flat_map(serde_json::from_str).collect_vec();

    let ordered_index_sum = packets.iter().tuples().enumerate()
        .filter_map(|(i, (p1, p2))| compare(p1, p2).is_lt().then_some(i + 1))
        .sum::<usize>();
    advtools::verify("Sum of correct indices", ordered_index_sum, 5529);

    let div1 = List(vec![List(vec![Int(2)])]);
    let div2 = List(vec![List(vec![Int(6)])]);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort_by(compare);
    let decoder_key = packets.iter().enumerate()
        .filter_map(|(i, p)| (p == &div1 || p == &div2).then_some(i + 1))
        .product::<usize>();
    advtools::verify("Decoder key", decoder_key, 27690);
}
