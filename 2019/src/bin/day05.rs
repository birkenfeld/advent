use advtools::prelude::Itertools;
use advtools::input::input_string;
use advent19::{Machine, Int};

const INPUT_1: Int = 1;
const INPUT_2: Int = 5;

fn main() {
    let cells = input_string().trim().split(',').map(|v| v.parse().unwrap()).collect_vec();

    let out = Machine::new(&cells, None).run(Some(INPUT_1)).1;
    advtools::print("First round", out.last().unwrap());

    let out = Machine::new(&cells, None).run(Some(INPUT_2)).1;
    advtools::print("Second round", out.last().unwrap());
}
