use advtools::prelude::{HashMap, Itertools};
use advtools::input;

const TARGET_ROOM: &str = "northpole object storage";
const LINE_FMT: &str = r"(.*)-(\d+)\[(.*)\]";

fn main() {
    let mut np_sector = 0;
    let mut sector_sum = 0;
    for (name, sector, checksum) in input::rx_lines::<(&str, u32, &str)>(LINE_FMT) {
        // count the digits in the name
        let mut counts = HashMap::new();
        for ch in name.chars().filter(|&ch| ch != '-') {
            *counts.entry(ch).or_insert(0) += 1;
        }
        // get letters sorted by count, then by alphabet
        let counts = counts.into_iter().map(|(ch, count)| (-count, ch)).sorted();

        // determine checksum from letter counts
        let real_checksum = counts.take(5).map(|x| x.1);
        if real_checksum.eq(checksum.chars()) {
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
    advtools::verify("Sector ID sum", sector_sum, 278221);
    advtools::verify("North Pole sector", np_sector, 267);
}
