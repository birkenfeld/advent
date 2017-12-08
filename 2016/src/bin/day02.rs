extern crate advtools;
extern crate strum;
#[macro_use] extern crate strum_macros;

use advtools::prelude::*;

#[derive(EnumString, Clone, Copy, Debug)]
enum Direction { U, R, D, L }
use Direction::*;

type Button = i32;

trait Keypad {
    fn next(Button, Direction) -> Button;
}

struct NormalKeypad;
struct FancyKeypad;

impl Keypad for NormalKeypad {
    // 1 2 3
    // 4 5 6
    // 7 8 9
    fn next(btn: Button, dir: Direction) -> Button {
        match (btn, dir) {
            (1...3, U) |
            (7...9, D) |
            (3, R) | (6, R) | (9, R) |
            (1, L) | (4, L) | (7, L) => btn,

            (4...9, U) => btn - 3,
            (1...6, D) => btn + 3,
            (1...2, R) | (4...5, R) | (7...8, R) => btn + 1,
            (2...3, L) | (5...6, L) | (8...9, L) => btn - 1,
            _ => panic!("invalid next button: {:?}, {:?}", btn, dir)
        }
    }
}

impl Keypad for FancyKeypad {
    //       1
    //    2  3  4
    // 5  6  7  8  9
    //   10 11 12
    //      13
    fn next(btn: Button, dir: Direction) -> Button {
        match (btn, dir) {
            (5, U) | (2, U) | (1, U) | (4, U) | (9, U) |
            (5, D) | (10, D) | (13, D) | (12, D) | (9, D) |
            (1, R) | (4, R) | (9, R) | (12, R) | (13, R) |
            (1, L) | (2, L) | (5, L) | (10, L) | (13, L)  => btn,

            (6...8, U) | (10...12, U)                     => btn - 4,
            (3, U) | (13, U)                              => btn - 2,

            (2...4, D) | (6...8, D)                       => btn + 4,
            (1, D) | (11, D)                              => btn + 2,

            (2...3, R) | (5...8, R) | (10...11, R)        => btn + 1,

            (3...4, L) | (6...9, L) | (11...12, L)        => btn - 1,

            _ => panic!("invalid next button: {:?}, {:?}", btn, dir)
        }
    }
}

fn find_code<K: Keypad>() -> String {
    let mut btn = 5;
    let code = iter_input::<String>().map(|line| {
        for ch in line.chars() {
            let dir = ch.to_string().parse().unwrap();
            btn = K::next(btn, dir);
        }
        btn
    });
    format!("{:X}", code.format(""))
}


fn main() {
    println!("Code (normal keypad): {}", find_code::<NormalKeypad>());
    println!("Code (fancy keypad): {}", find_code::<FancyKeypad>());
}
