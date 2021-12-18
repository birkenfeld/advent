use advtools::prelude::{rotate_left, rotate_right};
use advtools::input;

const INITIAL: &str = "abcdefgh";
const FINAL:   &str = "fbgdceah";
const FORMAT:  &str = "rotate left (.) steps?|\
                       rotate right (.) steps?|\
                       rotate based on position of letter (.)|\
                       swap position (.) with position (.)|\
                       swap letter (.) with letter (.)|\
                       reverse positions (.) through (.)|\
                       move position (.) to position (.)";

#[derive(Clone, Copy, Debug)]
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
    fn exec(self, v: &mut [char], forward: bool) {
        match (self, forward) {
            (Instr::RotL(n), true) | (Instr::RotR(n), false) => rotate_left(v, n),
            (Instr::RotR(n), true) | (Instr::RotL(n), false) => rotate_right(v, n),
            (Instr::SwapPos(i1, i2), _) => v.swap(i1, i2),
            (Instr::SwapLetter(c1, c2), _) => for ch in v {
                if *ch == c1 { *ch = c2; }
                else if *ch == c2 { *ch = c1; }
            },
            (Instr::Reverse(i1, i2), _) => v[i1..=i2].reverse(),
            (Instr::RotLetter(c), true) => {
                let mut n = v.iter().position(|&ch| ch == c).unwrap();
                n += if n >= 4 { 2 } else { 1 };
                rotate_right(v, n % v.len());
            },
            (Instr::RotLetter(c), false) => {
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
            (Instr::Move(i1, i2), true) => if i1 < i2 {
                rotate_left(&mut v[i1..=i2], 1);
            } else {
                rotate_right(&mut v[i2..=i1], 1);
            },
            (Instr::Move(i1, i2), false) => if i1 < i2 {
                rotate_right(&mut v[i1..=i2], 1);
            } else {
                rotate_left(&mut v[i2..=i1], 1);
            },
        }
    }
}

fn main() {
    let mut recipe = Vec::new();
    for (rot_l, rot_r, rot_let, sw_pos, sw_let, rev, mov) in input::rx_lines(FORMAT) {
        recipe.push(if let Some(steps) = rot_l {
            Instr::RotL(steps)
        } else if let Some(steps) = rot_r {
            Instr::RotR(steps)
        } else if let Some(letter) = rot_let {
            Instr::RotLetter(letter)
        } else if let Some((p1, p2)) = sw_pos {
            Instr::SwapPos(p1, p2)
        } else if let Some((l1, l2)) = sw_let {
            Instr::SwapLetter(l1, l2)
        } else if let Some((p1, p2)) = rev {
            Instr::Reverse(p1, p2)
        } else if let Some((p1, p2)) = mov {
            Instr::Move(p1, p2)
        } else {
            unreachable!()
        });
    }

    let mut password: Vec<char> = INITIAL.chars().collect();
    for instr in &recipe {
        instr.exec(&mut password, true);
    }
    advtools::verify("scrambled password", password.into_iter().collect::<String>(), "hcdefbag");

    password = FINAL.chars().collect();
    for instr in recipe.iter().rev() {
        instr.exec(&mut password, false);
    }
    advtools::verify("unscrambled password", password.into_iter().collect::<String>(), "fbhaegdc");
}
