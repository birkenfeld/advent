#![feature(io)]

use std::fs::File;
use std::io::Read;

fn main() {
    let fp = File::open("input.txt").unwrap();
    let res = fp.chars().enumerate().fold((0, None), |(level, basement), (i, ch)| {
        match ch.unwrap() {
            '(' => (level + 1, basement),
            ')' => (level - 1, if level == 0 { basement.or(Some(i + 1)) } else { basement }),
            _   => (level, basement),
        }
    });
    println!("Resulting floor: {}", res.0);
    println!("Basement: {}", res.1.unwrap());
}
