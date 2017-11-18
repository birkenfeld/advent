extern crate advtools;
extern crate itertools;

use itertools::Itertools;

const INPUT: &[u8] = b"00101000101111010";
const LEN: usize = 272;
const LEN2: usize = 35_651_584;

fn checksum(s: Vec<u8>) -> Vec<u8> {
    s.into_iter().tuples().map(|(a, b)| if a == b { b'1' } else { b'0' }).collect()
}

fn dragon(mut s: Vec<u8>) -> Vec<u8> {
    let ext = s.iter().rev().map(|&a| if a == b'1' { b'0' } else { b'1' }).collect_vec();
    s.push(b'0');
    s.extend(ext);
    s
}

fn process(target_len: usize) -> String {
    let mut s = INPUT.to_vec();
    s.reserve(target_len);
    while s.len() < target_len {
        s = dragon(s);
    }
    s.truncate(target_len);
    while s.len() % 2 == 0 {
        s = checksum(s);
    }
    String::from_utf8(s).unwrap()
}

fn main() {
    println!("Checksum, length {}: {}", LEN, process(LEN));
    println!("Checksum, length {}: {}", LEN2, process(LEN2));
}
