extern crate advtools;
extern crate permutohedron;

use std::collections::HashMap;
use std::iter::once;
use advtools::IterExt;
use permutohedron::Heap;

fn most_happiness(n: usize, table: &[[i16; 9]; 9]) -> i16 {
    Heap::new(&mut (0..n).collect::<Vec<_>>()).map(|p|
        once(&p[n-1]).chain(p.iter()).zip(p.iter()).map(
            |(&p1, &p2)| table[p1][p2] + table[p2][p1]).sum_from(0)).max().unwrap()
}

type S = String;
type InputLine = (S, S, S, i16, S, S, S, S, S, S, S);

fn main() {
    let mut table = [[0i16; 9]; 9];
    let mut map = HashMap::new();
    for row in advtools::iter_input::<InputLine>() {
        let (p1, _, verb, mut val, _, _, _, _, _, _, p2) = row;
        let p2 = p2.trim_matches('.').to_owned();
        if verb == "lose" {
            val = -val;
        }
        let n = map.len();
        let p1_id = *map.entry(p1).or_insert(n);
        let n = map.len();
        let p2_id = *map.entry(p2).or_insert(n);
        table[p1_id][p2_id] = val;
    }
    println!("Most happiness: {}", most_happiness(8, &table));
    println!("Most happiness including self: {}", most_happiness(9, &table));
}
