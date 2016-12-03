extern crate advtools;

#[derive(Clone, Copy, Debug)]
enum Direction { U, R, D, L }
use Direction::*;

#[derive(Clone, Copy)]
struct Button(i32);

impl Button {
    // 1 2 3
    // 4 5 6
    // 7 8 9
    fn next(self, dir: Direction) -> Self {
        match (self.0, dir) {
            (1...3, U) => self,
            (4...9, U) => Button(self.0 - 3),
            (7...9, D) => self,
            (1...6, D) => Button(self.0 + 3),
            (3, R) | (6, R) | (9, R) => self,
            (1...2, R) | (4...5, R) | (7...8, R) => Button(self.0 + 1),
            (1, L) | (4, L) | (7, L) => self,
            (2...3, L) | (5...6, L) | (8...9, L) => Button(self.0 - 1),
            _ => panic!("invalid next button: {:?}, {:?}", self.0, dir)
        }
    }
    //       1
    //    2  3  4
    // 5  6  7  8  9
    //   10 11 12
    //      13
    fn next_fancy(self, dir: Direction) -> Self {
        match (self.0, dir) {
            (5, U) | (2, U) | (1, U) | (4, U) | (9, U)    => self,
            (6...8, U) | (10...12, U)                     => Button(self.0 - 4),
            (3, U) | (13, U)                              => Button(self.0 - 2),

            (5, D) | (10, D) | (13, D) | (12, D) | (9, D) => self,
            (2...4, D) | (6...8, D)                       => Button(self.0 + 4),
            (1, D) | (11, D)                              => Button(self.0 + 2),

            (1, R) | (4, R) | (9, R) | (12, R) | (13, R)  => self,
            (2...3, R) | (5...8, R) | (10...11, R)        => Button(self.0 + 1),

            (1, L) | (2, L) | (5, L) | (10, L) | (13, L)  => self,
            (3...4, L) | (6...9, L) | (11...12, L)        => Button(self.0 - 1),

            _ => panic!("invalid next button: {:?}, {:?}", self.0, dir)
        }
    }
}


fn main() {
    let mut code_simple = String::new();
    let mut code_fancy = String::new();
    let mut btn_simple = Button(5);
    let mut btn_fancy = Button(5);
    for line in advtools::iter_input::<String>() {
        for ch in line.chars() {
            let dir = match ch {
                'U' => U,
                'R' => R,
                'D' => D,
                'L' => L,
                _ => panic!("invalid direction")
            };
            btn_simple = btn_simple.next(dir);
            btn_fancy = btn_fancy.next_fancy(dir);
        }
        code_simple.push_str(&format!("{:X}", btn_simple.0));
        code_fancy.push_str(&format!("{:X}", btn_fancy.0));
    }
    println!("Code (simple keypad): {}", code_simple);
    println!("Code (fancy keypad): {}", code_fancy);
}
