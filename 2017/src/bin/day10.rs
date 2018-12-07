use advtools::prelude::Itertools;
use advtools::input::{input_string, to_u8};

fn main() {
    let input_str = input_string().trim().to_owned();
    let input = input_str.split(',').map(to_u8).collect_vec();

    // Part 1: Preliminary knot hash.
    let marks = advent17::knot_process(&input, 1);
    println!("Product of first elements: {}", (marks[0] as u16) * (marks[1] as u16));

    // Part 2: Full knot hash of the input.
    let hash = advent17::knot_hash(input_str);
    println!("Knot hash: {:02x}", hash.into_iter().format(""));
}
