use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap()).lines();
    let (total_paper, total_ribbon) = lines.fold((0, 0), |(paper, ribbon), line| {
        let line = line.unwrap();
        let mut dims: Vec<usize> = line.split('x').map(|s| s.parse().unwrap()).collect();
        dims.sort();
        let (l, w, h) = (dims[0], dims[1], dims[2]);
        (paper + 2 * (l*w + w*h + h*l) + l*w, ribbon + l*w*h + 2 * (l + w))
    });
    println!("Paper: {}", total_paper);
    println!("Ribbon: {}", total_ribbon);
}
