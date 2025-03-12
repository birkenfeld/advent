use advtools::input;
use advtools::prelude::{HashMap, HashSet, Itertools};

fn main() {
    let mut antennae = HashMap::new();
    let mut w = 0;
    let mut h = 0;
    for (y, line) in input::lines().enumerate() {
        w = line.len() as i32;
        for (x, ch) in line.chars().enumerate() {
            if ch.is_alphanumeric() {
                antennae.entry(ch).or_insert_with(Vec::new).push((x as i32, y as i32));
            }
        }
        h = y as i32 + 1;
    }

    let mut locations = HashSet::new();
    for locs in antennae.values() {
        for ((ax, ay), (bx, by)) in locs.iter().tuple_combinations() {
            let n1x = 2*ax - bx;
            let n1y = 2*ay - by;
            let n2x = 2*bx - ax;
            let n2y = 2*by - ay;
            if n1x >= 0 && n1x < w && n1y >= 0 && n1y < h {
                locations.insert((n1x, n1y));
            }
            if n2x >= 0 && n2x < w && n2y >= 0 && n2y < h {
                locations.insert((n2x, n2y));
            }
        }
    }

    advtools::verify("Tiles with antinodes", locations.len(), 228);
}
