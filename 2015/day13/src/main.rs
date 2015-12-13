#![feature(iter_arith)]

extern crate permutohedron;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::once;
use permutohedron::Heap;

fn most_happiness(n: usize, table: &[[i16; 9]; 9]) -> i16 {
    Heap::new(&mut (0..n).collect::<Vec<_>>()).map(|p|
        once(&p[n-1]).chain(p.iter()).zip(p.iter()).map(
            |(&p1, &p2)| table[p1][p2] + table[p2][p1]).sum()).max().unwrap()
}

fn main() {
    let mut table = [[0i16; 9]; 9];
    let mut map = HashMap::new();
    for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
        let line = line.unwrap();
        let tok = line.trim_matches('.').split_whitespace().collect::<Vec<_>>();
        let p1 = tok[0].to_owned();
        let p2 = tok[10].to_owned();
        let mut val = tok[3].parse::<i16>().unwrap();
        if tok[2] == "lose" {
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
