use advtools::input;
use advtools::prelude::HashSet;

fn main() {
    let input = input::string().as_bytes();

    let find = |n| input.windows(n).enumerate().find(|(_, win)| {
        win.iter().collect::<HashSet<_>>().len() == n
    }).map(|(i, _)| i + n).unwrap();

    advtools::verify("Start of packet", find(4), 1779);
    advtools::verify("Start of message", find(14), 2635);
}
