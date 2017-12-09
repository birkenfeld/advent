extern crate advtools;
use advtools::prelude::*;

fn find_insert_time(discs: &[(u32, u32)]) -> u32 {
    (0..).find(|t| {
        discs.iter().enumerate().all(|(i, &(len, pos))| (i as u32 + pos + 1 + t) % len == 0)
    }).unwrap()
}

fn main() {
    let mut discs = iter_input_parts_trim((3, 11), ".").collect_vec();
    println!("Time to insert: {}", find_insert_time(&discs));
    discs.push((11, 0));
    println!("Time to insert with new disc: {}", find_insert_time(&discs));
}
