extern crate crypto;
extern crate rayon;

use crypto::md5::Md5;
use crypto::digest::Digest;

const INPUT: &'static [u8] = b"yzbqklnj";
const N: usize = 10_000_000;

fn check(i: usize) {
    let mut buf = [0u8; 16];
    let mut hash = Md5::new();
    hash.input(INPUT);
    hash.input(format!("{}", i).as_bytes());
    hash.result(&mut buf);
    if buf[0] | buf[1] == 0 {
        if buf[2] & 0xF0 == 0 {
            println!("Found 5-zero hash: {}", i);
            if buf[2] == 0 {
                println!("Found 6-zero hash: {}", i);
            }
        }
    }
}

fn check_parallel(from: usize, len: usize) {
    if len <= 1000 {
        for v in from..from+len {
            check(v);
        }
    } else {
        let half = len / 2;
        rayon::join(|| check_parallel(from, half),
                    || check_parallel(from + half, len - half));
    }
}

fn main() {
    check_parallel(0, N);
}
