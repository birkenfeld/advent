use advtools::prelude::Itertools;
use advtools::input::input_string;
use advent19::{Machine, Int};

const LANDING: Int = 19690720;

fn main() {
    let cells = input_string().trim().split(',').map(|v| v.parse().unwrap()).collect_vec();

    // Part 1: just run with a given noun/verb combination.
    advtools::print("First round", Machine::new(&cells, Some((12, 2))).run(None).0);

    // Part 2: try different nouns/verbs to get the desired landing date.
    for noun in 0..100 {
        for verb in 0..100 {
            if Machine::new(&cells, Some((noun, verb))).run(None).0 == LANDING {
                advtools::print("Second round", 100*noun + verb);
                break;
            }
        }
    }
}
