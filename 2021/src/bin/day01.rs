use advtools::itertools::Itertools;
use advtools::input::iter_input;

fn main() {
    let increases = iter_input::<u32>()
        .tuple_windows()
        .fold(0, |acc, (a, b)| acc + (b > a) as u32);
    advtools::verify("Increases single", increases, 1121);

    let increases = iter_input::<u32>()
        .tuple_windows()
        .fold(0, |acc, (a, _, _, b)| acc + (b > a) as u32);
    advtools::verify("Increases 3-window", increases, 1065);
}
