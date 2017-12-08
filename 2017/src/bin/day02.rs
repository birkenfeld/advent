extern crate advtools;
use advtools::prelude::*;

fn main() {
    let input = iter_input::<Vec<i32>>().collect_vec();
    let cksum1 = input.iter().map(|cols| cols.iter().minmax().into_option().unwrap())
                             .map(|(min, max)| max - min)
                             .sum::<i32>();
    let cksum2 = input.iter().map(|cols| {
        cols.iter().tuple_combinations().filter_map(|(a, b)| {
            if a % b == 0 { Some(a / b) }
            else if b % a == 0 { Some(b / a) }
            else { None }
        }).item()
    }).sum::<i32>();
    println!("Checksum 1: {}", cksum1);
    println!("Checksum 2: {}", cksum2);
}
