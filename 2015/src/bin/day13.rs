extern crate advtools;
extern crate permutohedron;

use advtools::prelude::*;
use std::iter::once;
use advtools::Uids;
use permutohedron::Heap;

fn most_happiness(n: usize, table: &[[i16; 9]; 9]) -> i16 {
    Heap::new(&mut (0..n).collect::<Vec<_>>()).map(|p|
        once(&p[n-1]).chain(p.iter()).zip(p.iter()).map(
            |(&p1, &p2)| table[p1][p2] + table[p2][p1]).sum()).max().unwrap()
}

fn main() {
    let mut table = [[0i16; 9]; 9];
    let mut map = Uids::new();
    for row in iter_input_parts((0, 2, 3, 10)) {
        let (p1, verb, mut val, p2): (String, String, i16, String) = row;
        let p2 = p2.trim_matches('.').to_owned();
        if verb == "lose" {
            val = -val;
        }
        let p1_id = map.get_id(p1);
        let p2_id = map.get_id(p2);
        table[p1_id][p2_id] = val;
    }
    println!("Most happiness: {}", most_happiness(8, &table));
    println!("Most happiness including self: {}", most_happiness(9, &table));
}
