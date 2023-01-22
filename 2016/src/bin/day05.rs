use std::char;
use md5::{Digest, Md5};
use advtools::rayon::prelude::*;
use advtools::input;

const LEN: usize = 8;
const BATCH: usize = 1_000_000;

fn check(input: &[u8], i: usize) -> Option<(usize, u8, u8)> {
    let mut ibuf = [0u8; 16];
    let mut hash = Md5::new();
    let n = itoa::write(&mut ibuf[..], i).unwrap();
    hash.update(input);
    hash.update(&ibuf[..n]);
    let buf = hash.finalize();
    if buf[0] | buf[1] == 0 && buf[2] & 0xF0 == 0 {
        Some((i, buf[2], buf[3] >> 4))
    } else {
        None
    }
}

fn main() {
    let input = input::string().as_bytes();
    // the first passcode can be collected directly
    let mut pass_door1 = String::new();
    // the second passcode needs to be constructed char by char
    let mut pass_door2 = vec![None; LEN];
    let mut n = 0;
    // we need more batches as long as not all pass_door2 digits are found
    // (pass_door1 is long done by then)
    while pass_door2.iter().any(|v| v.is_none()) {
        // get every candidate digit (with 00000 MD5 prefix) in the next batch
        let mut digits: Vec<_> = (n..n + BATCH).into_par_iter()
                                               .filter_map(|v| check(input, v)).collect();
        digits.sort_unstable();  // by n, then d6, then d7
        // update passcode for first door, just by order of number
        pass_door1.extend(digits.iter().flat_map(|d| char::from_digit(d.1 as u32, 16)));
        // update passcode for second door, where the index is d6, the code digit
        // is d7, and the first one wins
        for (_, d6, d7) in digits {
            let d6 = d6 as usize;
            if d6 < LEN && pass_door2[d6].is_none() {
                pass_door2[d6] = char::from_digit(d7.into(), 16);
            }
        }
        n += BATCH;
    }
    let pass_door1 = &pass_door1[..LEN];
    let pass_door2 = pass_door2.into_iter().flatten().collect::<String>();
    advtools::verify("First door", pass_door1, "1a3099aa");
    advtools::verify("Second door", pass_door2, "694190cd");
}
