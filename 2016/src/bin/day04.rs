extern crate advtools;
use advtools::prelude::{HashMap, Itertools};
use advtools::input::iter_input_regex;

const TARGET_ROOM: &str = "northpole object storage";
const LINE_FMT: &str = r"(.*)-(\d+)\[(.*)\]";

fn main() {
    let mut np_sector = 0;
    let mut sector_sum = 0;
    for (name, sector, checksum) in iter_input_regex::<(String, u32, String)>(LINE_FMT) {
        // count the digits in the name
        let mut counts = HashMap::default();
        for ch in name.chars().filter(|&ch| ch != '-') {
            *counts.entry(ch).or_insert(0) += 1;
        }
        // get letters sorted by count, then by alphabet
        let counts = counts.into_iter().map(|(ch, count)| (-count, ch)).sorted();

        // determine checksum from letter counts
        let real_checksum = counts.into_iter().take(5).map(|x| x.1).collect::<String>();
        if real_checksum == checksum {
            sector_sum += sector;

            // decode real name of room
            let decode = |ch| if ch == '-' { ' ' } else {
                let letter = (ch as u8) - b'a';
                let rotated = (letter + (sector % 26) as u8) % 26;
                (rotated + b'a') as char
            };
            if TARGET_ROOM.chars().eq(name.chars().map(decode)) {
                np_sector = sector;
            }
        }
    }
    println!("Sector ID sum: {}", sector_sum);
    println!("North Pole sector: {}", np_sector);
}
