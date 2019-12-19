use advtools::prelude::{Itertools, HashMap};
use advtools::input::input_string;

/// Step through the positions of a single wire, and call the given
/// function for each position and number of steps to reach it.
fn follow<F: FnMut(i16, i16, i32)>(instr: &str, mut visit: F) {
    let (mut x, mut y, mut s) = (0, 0, 0);
    for dir in instr.split(',') {
        for _ in 0..dir[1..].parse().unwrap() {
            s += 1;
            match &dir[..1] {
                "R" => x += 1,
                "L" => x -= 1,
                "D" => y += 1,
                "U" => y -= 1,
                _ => unreachable!()
            }
            visit(x, y, s);
        }
    }
}

fn main() {
    let input = input_string();
    let (w1, w2) = input.split('\n').next_tuple().unwrap();

    let mut coords = HashMap::new();
    let mut min_coords = i16::max_value();
    let mut min_steps = i32::max_value();

    // Follow the first wire, recording every visited coordinate with the number
    // of steps. If visited twice, keep the lower number.
    follow(&w1, |x, y, s1| { coords.entry((y, x)).or_insert(s1); });

    // Follow the second wire, tracking the minimum scores for both rounds for
    // each crossing.
    follow(&w2, |x, y, s2| if let Some(s1) = coords.get(&(y, x)) {
        min_coords = min_coords.min(x.abs() + y.abs());
        min_steps = min_steps.min(s1 + s2);
    });

    advtools::print("First round", min_coords);
    advtools::print("Second round", min_steps);
}
