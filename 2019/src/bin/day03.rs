use advtools::prelude::{Itertools, HashMap};
use advtools::input::input_string;
use advent19::Dir;

/// Step through the positions of a single wire, and call the given
/// function for each position and number of steps to reach it.
fn follow<F: FnMut((i16, i16), i32)>(instr: &str, mut visit: F) {
    let (mut xy, mut s) = ((0, 0), 0);
    for step in instr.split(',') {
        let dir = Dir::from_str(&step[..1]);
        for _ in 0..step[1..].parse().unwrap() {
            s += 1;
            xy = dir.step(xy);
            visit(xy, s);
        }
    }
}

fn main() {
    let input = input_string();
    let (w1, w2) = input.split('\n').next_tuple().unwrap();

    let mut coords = HashMap::new();
    let mut min_dist = i16::max_value();
    let mut min_steps = i32::max_value();

    // Follow the first wire, recording every visited coordinate with the number
    // of steps. If visited twice, keep the lower number.
    follow(&w1, |xy, s1| { coords.entry(xy).or_insert(s1); });

    // Follow the second wire, tracking the minimum scores for both rounds for
    // each crossing.
    follow(&w2, |xy, s2| if let Some(s1) = coords.get(&xy) {
        min_dist = min_dist.min(xy.0.abs() + xy.1.abs());
        min_steps = min_steps.min(s1 + s2);
    });

    advtools::print("Minimum manhattan distance", min_dist);
    advtools::print("Minimum number of steps", min_steps);
}
