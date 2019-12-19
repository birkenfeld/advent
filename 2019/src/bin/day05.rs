use advtools::prelude::Itertools;
use advtools::input::input_string;
use advent19::{Machine, Int};

const INPUT_1: Int = 1;
const INPUT_2: Int = 5;

fn main() {
    let cells = input_string().trim().split(',').map(|v| v.parse().unwrap()).collect_vec();

    let out = Machine::new(&cells, Some(INPUT_1)).last().unwrap();
    advtools::print("Output for program 1", out);

    let out = Machine::new(&cells, Some(INPUT_2)).last().unwrap();
    advtools::print("Output for program 5", out);
}
