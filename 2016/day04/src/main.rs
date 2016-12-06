extern crate advtools;
extern crate regex;
extern crate fnv;

const TARGET_ROOM: &'static str = "northpole object storage";
const LINE_FMT: &'static str = r"(.*)-(\d+)\[(.*)\]";

fn main() {
    let mut np_sector = 0;
    let mut sector_sum = 0;
    let rx = regex::Regex::new(LINE_FMT).unwrap();
    for line in advtools::iter_input::<String>() {
        let caps = rx.captures(&line).unwrap();

        // count the digits in the name
        let mut counts = fnv::FnvHashMap::default();
        for ch in caps[1].chars().filter(|&ch| ch != '-') {
            *counts.entry(ch).or_insert(0) += 1;
        }
        // get letters sorted by count, then by alphabet
        let counts = advtools::sorted(counts.into_iter().map(|(ch, count)| (-count, ch)));

        // determine checksum from letter counts
        let real_checksum = counts.into_iter().take(5).map(|x| x.1).collect::<String>();
        if real_checksum == caps[3] {
            let sector = caps[2].parse().unwrap();
            sector_sum += sector;

            // decode real name of room
            let decode = |ch| if ch == '-' { ' ' } else {
                let letter = (ch as u8) - b'a';
                let rotated = (letter + (sector % 26) as u8) % 26;
                (rotated + b'a') as char
            };
            if TARGET_ROOM.chars().eq(caps[1].chars().map(decode)) {
                np_sector = sector;
            }
        }
    }
    println!("Sector ID sum: {}", sector_sum);
    println!("North Pole sector: {}", np_sector);
}
