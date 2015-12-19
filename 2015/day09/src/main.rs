extern crate advtools;
extern crate permutohedron;

use std::cmp::{min, max};
use advtools::{IterExt, Uids};
use permutohedron::Heap;

type InputLine = (String, (), String, (), u16);

fn main() {
    let mut table = [[0u16; 8]; 8];
    let mut map = Uids::new();
    for (from, _, to, _, dist) in advtools::iter_input::<InputLine>() {
        let from_id = map.get_id(from);
        let to_id = map.get_id(to);
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
