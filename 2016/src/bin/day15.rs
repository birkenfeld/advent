extern crate advtools;

fn find_insert_time(discs: &[(u32, u32)]) -> u32 {
    (0..).find(|t| {
        discs.iter().enumerate().all(|(i, &(len, pos))| (i as u32 + pos + 1 + t) % len == 0)
    }).unwrap()
}

fn main() {
    let mut discs = Vec::<(u32, u32)>::new();
    for line in advtools::iter_input::<String>() {
        let mut parts = line.split_whitespace();
        let numpos = parts.nth(3).unwrap();
        let curpos = parts.nth(7).unwrap();
        discs.push((numpos.parse().unwrap(), curpos[..curpos.len()-1].parse().unwrap()));
    }
    println!("Time to insert: {}", find_insert_time(&discs));
    discs.push((11, 0));
    println!("Time to insert with new disc: {}", find_insert_time(&discs));
}
