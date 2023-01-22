use advtools::prelude::HashMap;
use advtools::input;
use advtools::grid::{Pos, Dir};

/// Step through the positions of a single wire, and call the given
/// function for each position and number of steps to reach it.
fn follow<F: FnMut(Pos, i32)>(instr: &str, mut visit: F) {
    let (mut pos, mut s) = (Pos(0, 0), 0);
    for step in instr.split(',') {
        let dir = Dir::from_str(&step[..1]);
        for _ in 0..step[1..].parse().unwrap() {
            s += 1;
            pos.step(dir);
            visit(pos, s);
        }
    }
}

fn main() {
    let (w1, w2) = input::parse();

    let mut coords = HashMap::new();
    let mut min_dist = i32::max_value();
    let mut min_steps = i32::max_value();

    // Follow the first wire, recording every visited coordinate with the number
    // of steps. If visited twice, keep the lower number.
    follow(w1, |pos, s1| { coords.entry(pos).or_insert(s1); });

    // Follow the second wire, tracking the minimum scores for both rounds for
    // each crossing.
    follow(w2, |pos, s2| if let Some(s1) = coords.get(&pos) {
        min_dist = min_dist.min(pos.manhattan());
        min_steps = min_steps.min(s1 + s2);
    });

    advtools::verify("Minimum manhattan distance", min_dist, 896);
    advtools::verify("Minimum number of steps", min_steps, 16524);
}
