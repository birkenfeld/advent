use advtools::prelude::{Itertools, HashMap};
use advtools::input::input_string;
use advent19::{Machine, binary_search};

fn main() {
    let code = Machine::parse(&input_string());

    let is_affected = |x, y| Machine::new(&code).with_input(x)
                                                .with_input(y).next() == Some(1);

    // Get an approximate ratio between x and y of the beam.
    let x1 = (10..15).find(|&x| is_affected(x, 10)).unwrap() - 1;
    let x2 = (10..15).rev().find(|&x| is_affected(x, 10)).unwrap() + 1;

    let count = (0..50).cartesian_product(0..50).filter(|&(x, y)| {
        x >= x1*y/10 && x <= x2*y/10 && is_affected(x, y)
    }).count();
    advtools::print("Affected tiles", count);

    // Find, by binary search, the correct y and x values.
    let mut xs = HashMap::new();
    let y = binary_search(0, 2000, |y| {
        let x = binary_search(x1*y/10, x2*y/10, |x| is_affected(x, y + 99));
        xs.insert(y, x);
        is_affected(x + 99, y)
    });
    advtools::print("Upper-left coordinate", xs[&y]*10000 + y);
}
