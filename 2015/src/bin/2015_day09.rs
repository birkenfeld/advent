use advtools::prelude::{Itertools, Uids};
use advtools::input;

fn main() {
    let mut table = [[0u16; 8]; 8];
    let mut map = Uids::new();
    for (from, to, dist) in input::rx_lines::<(&str, &str, u16)>("(.+) to (.+) = (.+)") {
        let from_id = map.get_id(from);
        let to_id = map.get_id(to);
        table[from_id.max(to_id)][from_id.min(to_id)] = dist;
        table[from_id.min(to_id)][from_id.max(to_id)] = dist;
    }
    let mut shortest = u16::max_value();
    let mut longest = 0;
    for p in (0..8).permutations(8) {
        let length = p.iter().zip(p.iter().skip(1)).map(|(p1, p2)| table[*p1][*p2]).sum();
        shortest = shortest.min(length);
        longest = longest.max(length);
    }
    advtools::verify("Shortest path", shortest, 117);
    advtools::verify("Longest path", longest, 909);
}
