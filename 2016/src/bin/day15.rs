use advtools::prelude::Itertools;
use advtools::input::iter_input_regex;

fn find_insert_time(discs: &[(u32, u32)]) -> u32 {
    (0..).find(|t| {
        discs.iter().enumerate().all(|(i, &(len, pos))| (i as u32 + pos + 1 + t) % len == 0)
    }).unwrap()
}

fn main() {
    let mut discs = iter_input_regex(".* has (\\d+) .* position (\\d+)").collect_vec();
    println!("Time to insert: {}", find_insert_time(&discs));
    discs.push((11, 0));
    println!("Time to insert with new disc: {}", find_insert_time(&discs));
}
