use std::sync::mpsc;
use md5::{Digest, Md5};
use advtools::rayon::prelude::*;
use advtools::input;

const N: u64 = 10_000_000;

fn check(input: &[u8], i: u64, tx: &mpsc::SyncSender<(u64, bool)>) {
    let mut ibuf = itoa::Buffer::new();
    let mut hash = Md5::new();
    hash.update(input);
    hash.update(&ibuf.format(i));
    let buf = hash.finalize();
    if (buf[0] | buf[1] == 0) && (buf[2] & 0xF0 == 0) {
        tx.send((i, buf[2] == 0)).unwrap();
    }
}

fn main() {
    let input = input::string().as_bytes();
    let (tx, rx) = mpsc::sync_channel(256);
    (0..N).into_par_iter().for_each(|n| check(input, n, &tx));
    drop(tx);
    let mut min5z = u64::max_value();
    let mut min6z = u64::max_value();
    while let Ok((number, sixzeros)) = rx.recv() {
        min5z = min5z.min(number);
        if sixzeros {
            min6z = min6z.min(number);
        }
    }
    advtools::verify("First 5-zero hash for", min5z, 282749);
    advtools::verify("First 6-zero hash for", min6z, 9962624);
}
