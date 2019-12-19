use advtools::prelude::Itertools;
use advtools::input::input_string;
use advent19::{Machine, Int};

const LANDING: Int = 19690720;

fn main() {
    let mut cells = input_string().trim().split(',').map(|v| v.parse().unwrap()).collect_vec();

    let mut run_with = |noun, verb| {
        cells[1] = noun;
        cells[2] = verb;
        let mut machine = Machine::new(&cells, None);
        machine.next();
        machine.mem(0)
    };

    // Part 1: just run with a given noun/verb combination.
    advtools::print("First round", run_with(12, 2));

    // Part 2: try different nouns/verbs to get the desired landing date.
    for noun in 0..100 {
        for verb in 0..100 {
            if run_with(noun, verb) == LANDING {
                advtools::print("Second round", 100*noun + verb);
                break;
            }
        }
    }
}
