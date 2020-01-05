use advtools::prelude::HashSet;
use advtools::input::{input_string, to_u32};
use advtools::grid::{Pos, Dir};

fn main() {
    let mut pos = Pos(0, 0);
    let mut dir = Dir::U;
    let mut visited = HashSet::new();
    let mut visited_twice = None;
    for instr in input_string().split(',') {
        let instr = instr.trim();
        dir = if instr.starts_with('R') { dir.right() } else { dir.left() };
        for _ in 0..to_u32(&instr[1..]) {
            pos.step(dir);
            if visited_twice.is_none() {
                if !visited.insert(pos) {
                    visited_twice = Some(pos);
                }
            }
        }
    }
    advtools::verify("Final distance", pos.manhattan(), 288);
    advtools::verify("Visited twice distance", visited_twice.unwrap().manhattan(), 111);
}
