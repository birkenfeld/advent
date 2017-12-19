extern crate advtools;
use advtools::prelude::*;

enum Dir { U, D, L, R }
use self::Dir::*;

fn main() {
    let map = input_string().lines().map(|s| s.as_bytes().to_vec()).collect_vec();

    let mut x = map[0].iter().position(|&c| c == b'|').unwrap();
    let mut y = 0;
    let mut dir = D;
    let mut path = Vec::new();
    let mut steps = 0;

    loop {
        match map[y][x] {
            b'+' => dir = match dir {
                L | R => if map[y-1][x] != b' ' { U } else { D },
                U | D => if map[y][x-1] != b' ' { L } else { R },
            },
            b'|' | b'-' => {},
            b' ' => break,
            c => path.push(c),
        }
        steps += 1;
        match dir {
            U => y -= 1,
            D => y += 1,
            L => x -= 1,
            R => x += 1,
        }
    }

    println!("Path: {}", from_utf8(path));
    println!("Steps: {}", steps);
}
