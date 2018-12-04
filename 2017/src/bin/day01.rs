extern crate advtools;
use advtools::prelude::Itertools;
use advtools::input::input_string;

fn main() {
    // Get input as a vector of chars.
    let input = input_string().trim().chars().collect_vec();
    // Pair each item with the `offset`th next item, wrapping around.
    let captcha = |offset| input.iter().zip(input.iter().cycle().skip(offset))
                                       .filter(|&(a, b)| a == b)
                                       .map(|(a, _)| a.to_digit(10).unwrap())
                                       .sum::<u32>();
    // Part 1: adjacent items.
    println!("First round: {}", captcha(1));
    // Part 2: "opposite" items on a ring.
    println!("Second round: {}", captcha(input.len() / 2));
}
