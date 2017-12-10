extern crate odds;
extern crate advtools;
use advtools::prelude::*;
use odds::slice::rotate_left;

fn process(input: &[u8], n: u32) -> Vec<u8> {
    let mut marks = (0..).take(256).collect_vec();
    let mut skip = 0;
    let mut total_rot = 0;
    for _ in 0..n {
        for &length in input {
            marks[..length as usize].reverse();
            let pos = length.wrapping_add(skip);
            rotate_left(&mut marks, pos as usize);
            total_rot += pos as usize;
            skip += 1;
        }
    }
    rotate_left(&mut marks, 256 - total_rot % 256);
    marks
}

fn main() {
    let input_str = input_string();
    let input = input_str.trim().split(',').map(to_u8).collect_vec();
    let marks = process(&input, 1);
    println!("Product of first elements: {}", (marks[0] as u16) * (marks[1] as u16));

    let mut input = input_str.trim().as_bytes().to_vec();
    input.extend(&[17, 31, 73, 47, 23]);
    let sparse = process(&input, 64);
    let dense = sparse.chunks(16).map(|v| v.iter().fold(0, |a, b| a^b));
    println!("Knot hash: {:02x}", dense.format(""));
}
