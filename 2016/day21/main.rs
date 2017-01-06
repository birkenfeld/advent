extern crate odds;
extern crate advtools;

use odds::slice::rotate_left;

const INITIAL: &'static str = "abcdefgh";
const FINAL:   &'static str = "fbgdceah";

#[derive(Debug)]
enum Instr {
    RotL(usize),
    RotR(usize),
    RotLetter(char),
    SwapPos(usize, usize),
    SwapLetter(char, char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Instr {
    fn exec(&self, v: &mut [char], forward: bool) {
        let sz = v.len();
        match (self, forward) {
            (&Instr::RotL(n), true) | (&Instr::RotR(n), false) => rotate_left(v, n),
            (&Instr::RotR(n), true) | (&Instr::RotL(n), false) => rotate_left(v, sz - n),
            (&Instr::SwapPos(i1, i2), _) => v.swap(i1, i2),
            (&Instr::SwapLetter(c1, c2), _) => for ch in v {
                if *ch == c1 { *ch = c2; }
                else if *ch == c2 { *ch = c1; }
            },
            (&Instr::Reverse(i1, i2), _) => v[i1..i2+1].reverse(),
            (&Instr::RotLetter(c), true) => {
                let mut n = v.iter().position(|&ch| ch == c).unwrap();
                n += if n >= 4 { 2 } else { 1 };
                rotate_left(v, sz - (n % sz));
            },
            (&Instr::RotLetter(c), false) => {
                let mut n = v.iter().position(|&ch| ch == c).unwrap();
                if n == 0 {
                    n = 1;
                } else if n % 2 == 0 {
                    n = 5 + n/2;
                } else {
                    n = 1 + n/2;
                }
                rotate_left(v, n);
            }
            (&Instr::Move(i1, i2), true) => if i1 < i2 {
                rotate_left(&mut v[i1..i2+1], 1);
            } else {
                rotate_left(&mut v[i2..i1+1], i1-i2);  // right by 1
            },
            (&Instr::Move(i1, i2), false) => if i1 < i2 {
                rotate_left(&mut v[i1..i2+1], i2-i1);
            } else {
                rotate_left(&mut v[i2..i1+1], 1);
            },
        }
    }
}

fn main() {
    let mut recipe = Vec::new();
    for line in advtools::iter_input::<String>() {
        let mut parts = line.split_whitespace();
        recipe.push(
            if line.starts_with("rotate left") {
                Instr::RotL(parts.nth(2).unwrap().parse().unwrap())
            } else if line.starts_with("rotate right") {
                Instr::RotR(parts.nth(2).unwrap().parse().unwrap())
            } else if line.starts_with("rotate based") {
                Instr::RotLetter(parts.nth(6).unwrap().chars().next().unwrap())
            } else if line.starts_with("swap position") {
                Instr::SwapPos(
                    parts.nth(2).unwrap().parse().unwrap(),
                    parts.nth(2).unwrap().parse().unwrap()
                )
            } else if line.starts_with("swap letter") {
                Instr::SwapLetter(
                    parts.nth(2).unwrap().chars().next().unwrap(),
                    parts.nth(2).unwrap().chars().next().unwrap()
                )
            } else if line.starts_with("reverse positions") {
                Instr::Reverse(
                    parts.nth(2).unwrap().parse().unwrap(),
                    parts.nth(1).unwrap().parse().unwrap()
                )
            } else if line.starts_with("move position") {
                Instr::Move(
                    parts.nth(2).unwrap().parse().unwrap(),
                    parts.nth(2).unwrap().parse().unwrap()
                )
            } else {
                panic!("invalid instruction line: {}", line);
            }
        );
    }

    let mut password: Vec<char> = INITIAL.chars().collect();
    for instr in &recipe {
        instr.exec(&mut password, true);
    }
    println!("scrambled password: {}", password.iter().cloned().collect::<String>());

    password = FINAL.chars().collect();
    for instr in recipe.iter().rev() {
        instr.exec(&mut password, false);
    }
    println!("unscrambled password: {}", password.iter().cloned().collect::<String>());
}
