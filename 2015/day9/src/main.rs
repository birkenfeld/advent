#![feature(iter_arith)]

extern crate permutohedron;
extern crate csv;

use std::cmp::{min, max};
use std::collections::HashMap;
use permutohedron::Heap;

type InputLine = (String, String, String, char, u16);

fn main() {
    let mut table = [[0u16; 8]; 8];
    let mut map = HashMap::new();
    for row in csv::Reader::from_file("input.txt").unwrap().delimiter(b' ').decode() {
        let (from, _, to, _, dist): InputLine = row.unwrap();
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
        let length = p.iter().zip(p.iter().skip(1)).map(|(p1, p2)| table[*p1][*p2]).sum();
        shortest = if shortest == 0 { length } else { min(shortest, length) };
        longest = if longest == 0 { length } else { max(longest, length) };
    }
    println!("Shortest path: {}", shortest);
    println!("Longest path: {}", longest);
}
