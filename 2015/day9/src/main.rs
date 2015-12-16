extern crate advtools;
extern crate permutohedron;

use std::cmp::{min, max};
use std::collections::HashMap;
use advtools::IterExt;
use permutohedron::Heap;

type InputLine = (String, String, String, char, u16);

fn main() {
    let mut table = [[0u16; 8]; 8];
    let mut map = HashMap::new();
    for (from, _, to, _, dist) in advtools::iter_input::<InputLine>() {
        let n = map.len();
        let from_id = *map.entry(from).or_insert(n);
        let n = map.len();
        let to_id = *map.entry(to).or_insert(n);
        table[max(from_id, to_id)][min(from_id, to_id)] = dist;
        table[min(from_id, to_id)][max(from_id, to_id)] = dist;
    }
    let mut shortest = 0;
    let mut longest = 0;
    let mut vec = (0..8).collect::<Vec<_>>();
    for p in Heap::new(&mut vec) {
        let length = p.iter().zip(p.iter().skip(1)).map(|(p1, p2)| table[*p1][*p2]).sum_from(0);
        shortest = if shortest == 0 { length } else { min(shortest, length) };
        longest = if longest == 0 { length } else { max(longest, length) };
    }
    println!("Shortest path: {}", shortest);
    println!("Longest path: {}", longest);
}
