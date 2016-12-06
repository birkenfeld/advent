extern crate advtools;

use std::iter;
use std::collections::HashMap;

fn main() {
    let mut lines = advtools::iter_input::<String>();
    let first_line = lines.next().unwrap();
    let mut maps = first_line.chars().map(|ch| iter::once((ch, 1)).collect())
                                     .collect::<Vec<HashMap<char, i32>>>();
    for line in lines {
        for (map, ch) in maps.iter_mut().zip(line.chars()) {
            *map.entry(ch).or_insert(0) += 1;
        }
    }
    let collect_by_freq = |weight| maps.iter().map(|map| {
        let freqs = advtools::sorted(map.iter().map(|(k, v)| (weight * v, k)));
        *freqs[0].1
    }).collect::<String>();
    println!("Message (most common): {}", collect_by_freq(-1));
    println!("Message (least common): {}", collect_by_freq(1));
}
