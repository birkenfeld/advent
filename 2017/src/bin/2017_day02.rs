use advtools::prelude::Itertools;
use advtools::input;

fn main() {
    let input = input::parse_vec::<Vec<i32>>();

    // Part 1: For each line, find difference between minimum and maximum and sum up.
    let cksum1 = input.iter().map(|cols| cols.iter().minmax().into_option().unwrap())
                             .map(|(min, max)| max - min)
                             .sum::<i32>();
    advtools::verify("Checksum 1", cksum1, 47136);

    // Part 2: For each line, find the combination of two items that are divisible
    // and sum up their quotient.
    let cksum2 = input.iter().map(|cols| {
        cols.iter().tuple_combinations().find_map(|(a, b)| {
            if a % b == 0 { Some(a / b) }
            else if b % a == 0 { Some(b / a) }
            else { None }
        }).unwrap()
    }).sum::<i32>();
    advtools::verify("Checksum 2", cksum2, 250);
}
