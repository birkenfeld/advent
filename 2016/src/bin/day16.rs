use advtools::prelude::Itertools;
use advtools::input::{input_string, from_utf8};
use advtools::rayon::prelude::*;

const LEN: usize = 272;
const LEN2: usize = 35_651_584;

fn checksum(s: Vec<u8>) -> Vec<u8> {
    s.par_chunks(2).map(|s| if s[0] == s[1] { b'1' } else { b'0' }).collect()
}

fn dragon(mut s: Vec<u8>) -> Vec<u8> {
    let ext = s.iter().rev().map(|&a| if a == b'0' { b'1' } else { b'0' }).collect_vec();
    s.push(b'0');
    s.extend(ext);
    s
}

fn process(input: &[u8], target_len: usize) -> String {
    let mut s = input.to_vec();
    s.reserve(target_len);
    while s.len() < target_len {
        s = dragon(s);
    }
    s.truncate(target_len);
    while s.len() % 2 == 0 {
        s = checksum(s);
    }
    from_utf8(s)
}

fn main() {
    let input = input_string();
    let input = input.trim().as_bytes();
    advtools::verify("Checksum, short", process(input, LEN), "10010100110011100");
    advtools::verify("Checksum, long", process(input, LEN2), "01100100101101100");
}
