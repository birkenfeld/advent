extern crate advtools;
use advtools::prelude::{HashMap, Itertools};

const INPUT: u32 = 312051;

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
    // Part 1: Just walk the spiral.  (Could start from the largest full ring, but this
    // is very quick anyway.)
    let mut pos = (0, 0);
    for _ in 1..INPUT {
        pos = next_pos(pos);
    }
    println!("Distance for {}: {}", INPUT, pos.0.abs() + pos.1.abs());

    // Part 2: Walk the spiral and insert all the calculated ambient sums in a map.
    let mut map = HashMap::default();
    let mut pos = (0, 0);
    map.insert(pos, 1);
    let value = loop {
        pos = next_pos(pos);
        // Collect the ambient sum from (x-1, x, x+1) x (y-1, y, y+1).
        let write = (-1..=1).cartesian_product(-1..=1)
                           .map(|d| map.get(&(pos.0 + d.0, pos.1 + d.1)).unwrap_or(&0))
                           .sum::<u32>();
        if write > INPUT {
            break write;
        }
        map.insert(pos, write);
    };
    println!("Value written: {}", value);
}
