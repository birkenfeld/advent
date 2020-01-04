use std::iter::repeat;
use advtools::prelude::Itertools;
use advtools::input::input_string;

fn main() {
    let instr = input_string();
    let input = instr.trim().chars().map(|ch| ch.to_digit(10).unwrap() as i32).collect_vec();
    let n = input.len();

    // Part 1: straight up do the algorithm as stated in the exercise.
    let mut digits = input.clone();
    // Precalculate the coefficient vectors for all positions.
    let coeff_vecs = (1..=n).map(|i| {
        repeat(0).take(i).chain(repeat(1).take(i))
                         .chain(repeat(0).take(i))
                         .chain(repeat(-1).take(i))
                         .cycle().skip(1).take(n).collect_vec()
    }).collect_vec();
    // Run the 100 rounds.
    for _ in 0..100 {
        digits = coeff_vecs.iter().map(|coeffs| {
            (digits.iter().zip(coeffs).map(|(x, y)| x * y).sum::<i32>() % 10).abs()
        }).collect();
    }
    advtools::verify("First 8 digits", digits.iter().take(8).join(""), 40921727);

    // Determine the offset for part 2.  Our quick way to solve this part
    // only works with offsets that are in the second half of the input.
    let offset: usize = instr[..7].parse().unwrap();
    assert!(offset >= 5000*n);

    // Take all the digits in the 10000x input *following* the offset,
    // but reverse the digits.
    let n2 = 10000*n - offset;
    let mut digits = input.iter().rev().cloned().cycle().take(n2).collect_vec();
    // For the second half of any input, each digit becomes the sum (modulo 10)
    // of it and all the following digits, since they each see a "1" as the
    // coefficient.  This can be efficiently calculated using a scan iterator
    // over the reversed digits that keeps track of a running sum.
    for _ in 0..100 {
        digits = digits.iter().scan(0, |sum, x| { *sum += x; Some(*sum % 10) }).collect();
    }
    // To get the first 8 digits, we have to take from the end.
    advtools::verify("First 8 digits", digits.iter().rev().take(8).join(""), 89950138);
}
