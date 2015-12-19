extern crate advtools;
extern crate permutohedron;

use std::iter::once;
use advtools::{IterExt, Uids};
use permutohedron::Heap;

fn most_happiness(n: usize, table: &[[i16; 9]; 9]) -> i16 {
    Heap::new(&mut (0..n).collect::<Vec<_>>()).map(|p|
        once(&p[n-1]).chain(p.iter()).zip(p.iter()).map(
            |(&p1, &p2)| table[p1][p2] + table[p2][p1]).sum_from(0)).max().unwrap()
}

type InputLine = (String, (), String, i16, [(); 6], String);

fn main() {
    let mut table = [[0i16; 9]; 9];
    let mut map = Uids::new();
    for row in advtools::iter_input::<InputLine>() {
        let (p1, _, verb, mut val, _, p2) = row;
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
