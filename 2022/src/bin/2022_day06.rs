use advtools::input;
use advtools::prelude::Itertools;

fn main() {
    let input = input::string().as_bytes();

    let find = |n| n + input.windows(n)
                            .position(|win| win.iter().all_unique())
                            .unwrap();

    advtools::verify("Start of packet", find(4), 1779);
    advtools::verify("Start of message", find(14), 2635);
}
