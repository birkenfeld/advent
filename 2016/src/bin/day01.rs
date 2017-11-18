extern crate advtools;

use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction { N, W, S, E }
use Direction::*;

impl Direction {
    fn left(self) -> Self {
        match self { N => W, W => S, S => E, E => N }
    }
    fn right(self) -> Self {
        match self { N => E, E => S, S => W, W => N }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(i32, i32);

impl Pos {
    fn walk(&mut self, d: Direction, n: i32) {
        match d {
            N => self.0 += n,
            S => self.0 -= n,
            E => self.1 += n,
            W => self.1 -= n,
        }
    }
    fn dist(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

fn main() {
    let mut pos = Pos(0, 0);
    let mut dir = Direction::N;
    let mut visited = HashSet::new();
    let mut visited_twice = None;
    for instr in advtools::input_string().split(',') {
        let instr = instr.trim();
        dir = if instr.starts_with('R') { dir.right() } else { dir.left() };
        for _ in 0..instr[1..].parse().unwrap() {
            pos.walk(dir, 1);
            if visited_twice.is_none() {
                if visited.contains(&pos) {
                    visited_twice = Some(pos);
                }
                visited.insert(pos);
            }
        }
    }
    println!("Final distance: {}", pos.dist());
    println!("Visited twice distance: {}", visited_twice.unwrap().dist());
}
