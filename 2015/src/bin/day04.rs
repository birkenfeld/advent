extern crate md5;
extern crate rayon;
extern crate itoa;

use std::sync::mpsc;
use md5::{Digest, Md5};
use rayon::prelude::*;

const INPUT: &[u8] = b"yzbqklnj";
const N: u64 = 10_000_000;

fn check(i: u64, tx: &mpsc::SyncSender<(u64, bool)>) {
    let mut ibuf = [0u8; 16];
    let mut hash = Md5::new();
    let n = itoa::write(&mut ibuf[..], i).unwrap();
    hash.input(INPUT);
    hash.input(&ibuf[..n]);
    let buf = hash.hash();
    if (buf[0] | buf[1] == 0) && (buf[2] & 0xF0 == 0) {
        tx.send((i, buf[2] == 0)).unwrap();
    }
}

fn main() {
    let (tx, rx) = mpsc::sync_channel(256);
    (0..N).into_par_iter().for_each(|n| check(n, &tx));
    drop(tx);
    let mut min5z = u64::max_value();
    let mut min6z = u64::max_value();
    while let Ok((number, sixzeros)) = rx.recv() {
        min5z = min5z.min(number);
        if sixzeros {
            min6z = min6z.min(number);
        }
    }
    println!("First 5-zero hash for: {}", min5z);
    println!("First 6-zero hash for: {}", min6z);
}
