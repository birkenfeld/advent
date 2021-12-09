use advtools::itertools::Itertools;
use advtools::input::iter_input;

fn main() {
    // Part 1: check every two successive numbers.
    let increases = iter_input::<u32>()
        .tuple_windows()
        .fold(0, |acc, (a, b)| acc + (b > a) as u32);
    advtools::verify("Increases single", increases, 1121);

    // Part 2: check successive triples, although only the first and fourth item
    // in each window play a role.
    let increases = iter_input::<u32>()
        .tuple_windows()
        .fold(0, |acc, (a, _, _, b)| acc + (b > a) as u32);
    advtools::verify("Increases 3-window", increases, 1065);
}
