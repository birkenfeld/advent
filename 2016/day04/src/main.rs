extern crate advtools;
extern crate fnv;

fn main() {
    let mut np_sector = 0;
    let mut sector_sum = 0;
    for line in advtools::iter_input::<String>() {
        // parse the input line
        let mut s1 = line.split('[');
        let mut parts = s1.next().unwrap().split('-');
        let checksum = &s1.next().unwrap()[0..5];
        let sector: u32 = parts.next_back().unwrap().parse().unwrap();
        let name_parts = parts.collect::<Vec<_>>();

        // count the digits in the name
        let mut counts = fnv::FnvHashMap::default();
        for part in &name_parts {
            for ch in part.chars() {
                *counts.entry(ch).or_insert(0) += 1;
            }
        }
        // get letters sorted by count, then by alphabet
        let mut sorted_counts = counts.into_iter().map(|(ch, count)| (-count, ch))
                                                  .collect::<Vec<_>>();
        sorted_counts.sort();

        // determine checksum from letter counts
        let real_checksum = sorted_counts.into_iter().take(5).map(|x| x.1)
                                                             .collect::<String>();
        if real_checksum == checksum {
            sector_sum += sector;

            // decode real name of room
            let mut real_name = String::with_capacity(line.len());
            for part in name_parts {
                for ch in part.chars() {
                    let letter = (ch as u8) - b'a';
                    let rotated = (letter + (sector % 26) as u8) % 26;
                    real_name.push((rotated + b'a') as char);
                }
                real_name.push(' ');
            }
            real_name.pop();  // final space
            if real_name == "northpole object storage" {
                np_sector = sector;
            }
        }
    }
    println!("Sector ID sum: {}", sector_sum);
    println!("North Pole sector: {}", np_sector);
}
