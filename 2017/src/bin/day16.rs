extern crate advtools;
extern crate odds;

use advtools::prelude::*;
use odds::slice::rotate_left;

enum Move {
    RotLeft(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

fn dance_one(dance: &[Move], dancers: &mut [u8]) {
    for move_ in dance {
        match *move_ {
            Move::RotLeft(n) => rotate_left(dancers, n),
            Move::Exchange(i, j) => dancers.swap(i, j),
            Move::Partner(a, b) => for place in dancers.iter_mut() {
                if *place == a { *place = b; }
                else if *place == b { *place = a; }
            }
        }
    }
}

fn as_string(dancers: &[u8]) -> String {
    dancers.iter().map(|b| (b + b'a') as char).collect()
}

fn main() {
    let dance = input_string().trim().split(',').map(|mov| {
        if mov.starts_with("s") {
            Move::RotLeft(16 - to_usize(&mov[1..]))
        } else if mov.starts_with("x") {
            let mut split = mov[1..].split('/');
            let pos1 = to_usize(split.item());
            let pos2 = to_usize(split.item());
            Move::Exchange(pos1, pos2)
        } else {
            let mut split = mov[1..].split('/');
            let prog1 = split.item().chars().item() as u8 - b'a';
            let prog2 = split.item().chars().item() as u8 - b'a';
            Move::Partner(prog1, prog2)
        }
    }).collect_vec();

    let mut seen = HashSet::new();
    let mut dancers = (0..16).collect_vec();
    while seen.insert(dancers.clone()) {
        dance_one(&dance, &mut dancers);
    }
    let cycle_len = seen.len();

    dance_one(&dance, &mut dancers);
    println!("Order after 1 dance: {}", as_string(&dancers));

    for _ in 0..(1_000_000_000 % cycle_len) - 1 {
        dance_one(&dance, &mut dancers);
    }
    println!("Order after 1 billion dances: {}", as_string(&dancers));
}
