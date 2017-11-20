extern crate md5;
extern crate rayon;
extern crate itoa;

use std::cmp::min;
use std::sync::mpsc;
use md5::{Digest, Md5};

const INPUT: &'static [u8] = b"yzbqklnj";
const N: usize = 10_000_000;

fn check(i: usize, tx: &mut mpsc::Sender<(usize, usize)>) {
    let mut ibuf = [0u8; 16];
    let mut hash = Md5::new();
    let n = itoa::write(&mut ibuf[..], i).unwrap();
    hash.input(INPUT);
    hash.input(&ibuf[..n]);
    let buf = hash.hash();
    if buf[0] | buf[1] == 0 {
        if buf[2] & 0xF0 == 0 {
            tx.send((5, i)).unwrap();
            if buf[2] == 0 {
                tx.send((6, i)).unwrap();
            }
        }
    }
}

fn check_parallel(from: usize, len: usize, mut tx: mpsc::Sender<(usize, usize)>) {
    if len <= 1000 {
        for v in from..from+len {
            check(v, &mut tx);
        }
    } else {
        let half = len / 2;
        let txc = tx.clone();
        rayon::join(|| check_parallel(from, half, txc),
                    || check_parallel(from + half, len - half, tx));
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    check_parallel(0, N, tx);
    let mut min5z = 0;
    let mut min6z = 0;
    while let Ok((nzeros, number)) = rx.recv() {
        if nzeros == 5 {
            min5z = if min5z == 0 { number } else { min(min5z, number) };
        } else if nzeros == 6 {
            min6z = if min6z == 0 { number } else { min(min6z, number) };
        }
    }
    println!("First 5-zero hash for: {}", min5z);
    println!("First 6-zero hash for: {}", min6z);
}
