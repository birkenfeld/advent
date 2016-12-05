extern crate crypto;
extern crate rayon;

use crypto::md5::Md5;
use crypto::digest::Digest;
use rayon::prelude::*;

const INPUT: &'static [u8] = b"uqwqemis";
const N: usize = 30_000_000;

fn check(i: usize) -> Option<(usize, u8, u8)> {
    let mut buf = [0u8; 16];
    let mut hash = Md5::new();
    hash.input(INPUT);
    hash.input(format!("{}", i).as_bytes());
    hash.result(&mut buf);
    if buf[0] | buf[1] == 0 && buf[2] & 0xF0 == 0 {
        Some((i, buf[2], buf[3] >> 4))
    } else {
        None
    }
}

fn main() {
    let mut cands1 = (0..N).into_par_iter().filter_map(check)
                                           .fold(|| Vec::new(), |mut v, x| { v.push(x); v })
                                           .reduce(|| Vec::new(), |mut v, x| { v.extend(x); v });
    let mut cands2 = cands1.iter().cloned().map(|(i, d6, d7)| (d6, i, d7)).collect::<Vec<_>>();
    cands1.sort();
    cands2.sort();
    let p1 = cands1.into_iter().take(8).map(|t| format!("{:x}", t.1)).collect::<String>();
    let mut p2 = vec![16; 8];
    for (i, _, d) in cands2 {
        if i >= 8 {
            break;
        }
        if p2[i as usize] == 16 {
            p2[i as usize] = d;
        }
    }
    let p2 = p2.into_iter().map(|x| format!("{:x}", x)).collect::<String>();
    println!("First door: {:?}", p1);
    println!("Second door: {:?}", p2);
}
