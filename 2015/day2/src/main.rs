use std::cmp::{min, max};
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let mut total_paper = 0;
    let mut total_ribbon = 0;
    for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
        let line = line.unwrap();
        let lens = line.split("x").map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
        let lw = lens[0] * lens[1];
        let wh = lens[1] * lens[2];
        let hl = lens[2] * lens[0];
        total_paper += 2 * (lw + wh + hl) + min(min(lw, wh), hl);
        total_ribbon += lens[0] * lens[1] * lens[2] +
            2 * (lens[0] + lens[1] + lens[2] - max(max(lens[0], lens[1]), lens[2]));
    }
    println!("Paper: {}", total_paper);
    println!("Ribbon: {}", total_ribbon);
}
