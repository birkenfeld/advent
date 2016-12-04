extern crate advtools;

use std::collections::HashMap;

fn main() {
    let mut np_sector = 0;
    let mut sector_sum = 0;
    for line in advtools::iter_input::<String>() {
        let mut s1 = line.split('[');
        let mut parts = s1.next().unwrap().split('-');
        let checksum = &s1.next().unwrap()[0..5];
        let sector: u32 = parts.next_back().unwrap().parse().unwrap();

        let name_parts = parts.collect::<Vec<_>>();
        let mut counts = HashMap::new();
        for part in &name_parts {
            for ch in part.chars() {
                *counts.entry(ch).or_insert(0) += 1;
            }
        }
        let mut sorted_counts = counts.into_iter().map(|(ch, count)| (-count, ch))
                                                  .collect::<Vec<_>>();
        sorted_counts.sort();
        let should_checksum = sorted_counts.into_iter().take(5)
                                                       .map(|x| x.1).collect::<String>();
        if should_checksum == checksum {
            sector_sum += sector;

            let mut real_name = String::new();
            for part in name_parts {
                for ch in part.chars() {
                    let letter = (ch as u8) - b'a';
                    let rotated = (letter + (sector % 26) as u8) % 26;
                    real_name.push((rotated + b'a') as char);
                }
                real_name.push(' ');
            }
            real_name.pop();
            if real_name == "northpole object storage" {
                np_sector = sector;
            }
        }
    }
    println!("Sector ID sum: {}", sector_sum);
    println!("North Pole sector: {}", np_sector);
}
