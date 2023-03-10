use advtools::prelude::HashSet;
use advtools::input;
use advtools::grid::{Pos, Dir};

fn main() {
    let mut pos = Pos(0, 0);
    let mut dir = Dir::U;
    let mut visited = HashSet::new();
    let mut visited_twice = None;
    for instr in input::string().split(", ") {
        dir = if instr.starts_with('R') { dir.right() } else { dir.left() };
        for _ in 0..instr[1..].parse().unwrap() {
            pos += dir;
            if visited_twice.is_none() && !visited.insert(pos) {
                visited_twice = Some(pos);
            }
        }
    }
    advtools::verify("Final distance", pos.manhattan(), 288);
    advtools::verify("Visited twice distance", visited_twice.unwrap().manhattan(), 111);
}
