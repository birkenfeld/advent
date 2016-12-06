extern crate advtools;

use std::collections::HashMap;

fn main() {
    let mut lines = advtools::iter_input::<String>();
    let mut all_maps = Vec::new();
    for ch in lines.next().unwrap().chars() {
        let mut map = HashMap::new();
        map.insert(ch, 1);
        all_maps.push(map);
    }
    for line in lines {
        for (map, ch) in all_maps.iter_mut().zip(line.chars()) {
            *map.entry(ch).or_insert(0) += 1;
        }
    }
    let collect_by_freq = |weight| all_maps.iter().map(|map| {
        let freqs = advtools::sorted(map.into_iter().map(|(k, v)| (weight * v, k)));
        *freqs[0].1
    }).collect::<String>();
    println!("Message (most common): {}", collect_by_freq(-1));
    println!("Message (least common): {}", collect_by_freq(1));
}
