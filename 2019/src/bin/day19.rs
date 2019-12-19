use advtools::prelude::{Itertools, HashMap};
use advtools::input::input_string;
use advent19::{Machine, binary_search};

fn main() {
    let code = Machine::parse(&input_string());

    let is_affected = |x, y| Machine::new(&code).with_input(x)
                                                .with_input(y).next() == Some(1);

    let count = (0..50).cartesian_product(0..50)
                       .filter(|&(x, y)| is_affected(x, y)).count();
    advtools::print("Affected tiles", count);

    let mut xs = HashMap::new();
    let y = binary_search(0, 2000, |y| {
        let x = binary_search(y, 1400*y/1000, |x| is_affected(x, y + 99));
        xs.insert(y, x);
        is_affected(x + 99, y)
    });
    advtools::print("Upper-left coordinate", xs[&y]*10000 + y);
}
