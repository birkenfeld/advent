extern crate md5;
extern crate rayon;
extern crate itoa;

use md5::{Digest, Md5};
use rayon::prelude::*;

const INPUT: &[u8] = b"ngcjuoqr";
const OFFSET: usize = 1000;
const KEYLEN: usize = 64;

const HEXCHARS: &[u8] = b"0123456789abcdef";

fn hash_to_hex(hash: Md5, sbuf: &mut [u8; 32]) {
    let buf = hash.result();
    for (i, &byte) in buf.iter().enumerate() {
        sbuf[2*i] = HEXCHARS[(byte >> 4) as usize];
        sbuf[2*i+1] = HEXCHARS[(byte & 0xf) as usize];
    }
}

fn digit(ch: u8) -> u8 {
    if ch >= b'a' { ch - b'a' + 10 } else { ch - b'0' }
}

fn find_multiples(i: usize, n: usize) -> Option<(usize, u32)> {
    let mut ibuf = [0; 16];
    let mut sbuf = [0; 32];
    let mut hash = Md5::new();
    let m = itoa::write(&mut ibuf[..], i).unwrap();
    hash.input(INPUT);
    hash.input(&ibuf[..m]);
    hash_to_hex(hash, &mut sbuf);
    for _ in 0..n {
        let mut hash = Md5::new();
        hash.input(&sbuf);
        hash_to_hex(hash, &mut sbuf);
    }
    // find the first triplet, mark in lower 16 bits
    (0..sbuf.len()-2).find(|&j| sbuf[j..j+3].iter().all(|&c| c == sbuf[j])).map(|j| {
        let mut res = 1 << digit(sbuf[j]);
        // find all quintuplets, mark in lower 16 bits
        for k in (0..sbuf.len()-4).filter(|&k| sbuf[k..k+5].iter().all(|&c| c == sbuf[k])) {
            res |= (1 << digit(sbuf[k])) << 16;
        }
        (i, res)
    })
}

fn collect_hashes(i1: usize, i2: usize, n: usize) -> Vec<(usize, u32)> {
    (i1..i2).into_par_iter().filter_map(|i| find_multiples(i, n)).collect::<Vec<_>>()
}

fn find_last_index(n: usize) -> usize {
    let mut i = OFFSET;
    let mut h1 = collect_hashes(0, i, n);
    let mut h2;
    let mut key = Vec::new();

    loop {
        h2 = collect_hashes(i, i+OFFSET, n);
        for &(j1, info1) in &h1 {
            let mask = info1 << 16;
            if h1.iter().chain(&h2).any(|&(j2, info2)| j2 > j1 && j2 <= j1 + 1001 && info2 & mask != 0) {
                key.push(j1);
                if key.len() >= KEYLEN {
                    return key[KEYLEN - 1];
                }
            }
        }
        h1 = h2;
        i += OFFSET;
    }
}

fn main() {
    println!("Last index: {}", find_last_index(0));
    println!("Last index (stretching): {}", find_last_index(2016));
}
