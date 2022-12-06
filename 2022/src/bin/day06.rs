use advtools::input;
use advtools::prelude::Itertools;

fn main() {
    let input = input::string().as_bytes();

    let find = |n| input.windows(n)
                        .enumerate()
                        .find(|(_, win)| win.iter().all_unique())
                        .map(|(i, _)| i + n).unwrap();

    advtools::verify("Start of packet", find(4), 1779);
    advtools::verify("Start of message", find(14), 2635);
}
