use advtools::prelude::{HashMap, Itertools};
use advtools::input::{input_string, to_u32};

/// Determine next coordinate of the position on the spiral.
fn next_pos((x, y): (i32, i32)) -> (i32, i32) {
    if x > 0 && (x.abs() > y.abs()) {
        (x, y + 1)
    } else if x < 0 && (x.abs() > y.abs() || x == -y) {
        (x, y - 1)
    } else if y > 0 && (y.abs() > x.abs() || x == y) {
        (x - 1, y)
    } else if y < 0 && (y.abs() > x.abs() || x == y) {
        (x + 1, y)
    } else {
        (x + 1, y)
    }
}

fn main() {
    let input = to_u32(input_string().trim());

    // Part 1: Just walk the spiral.  (Could start from the largest full ring,
    // but this is very quick anyway.)
    let mut pos = (0, 0);
    for _ in 1..input {
        pos = next_pos(pos);
    }
    advtools::verify("Distance", pos.0.abs() + pos.1.abs(), 430);

    // Part 2: Walk the spiral and insert all the calculated ambient sums in a map.
    let mut map = HashMap::new();
    let mut pos = (0, 0);
    map.insert(pos, 1);
    let value = loop {
        pos = next_pos(pos);
        // Collect the ambient sum from (x-1, x, x+1) x (y-1, y, y+1).
        let write = (-1..=1).cartesian_product(-1..=1)
                           .map(|d| map.get(&(pos.0 + d.0, pos.1 + d.1)).unwrap_or(&0))
                           .sum::<u32>();
        if write > input {
            break write;
        }
        map.insert(pos, write);
    };
    advtools::verify("Value written", value, 312453);
}
