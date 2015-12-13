#![feature(iter_arith)]

extern crate permutohedron;
extern crate csv;

use std::collections::HashMap;
use std::iter::once;
use permutohedron::Heap;

fn most_happiness(n: usize, table: &[[i16; 9]; 9]) -> i16 {
    Heap::new(&mut (0..n).collect::<Vec<_>>()).map(|p|
        once(&p[n-1]).chain(p.iter()).zip(p.iter()).map(
            |(&p1, &p2)| table[p1][p2] + table[p2][p1]).sum()).max().unwrap()
}

type S = String;
type InputLine = (S, S, S, i16, (S, S, S, S, S, S), S);

fn main() {
    let mut table = [[0i16; 9]; 9];
    let mut map = HashMap::new();
    for row in csv::Reader::from_file("input.txt").unwrap().delimiter(b' ').decode() {
        let (p1, _, verb, mut val, _, p2): InputLine = row.unwrap();
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
