use advtools::input::input_string;
use advtools::rayon::prelude::*;
use advent19::Machine;

const LANDING: i64 = 19690720;

fn main() {
    let code = Machine::parse(&input_string());

    let run_with = |noun, verb| {
        let mut machine = Machine::new(&code);
        machine.set_mem(1, noun);
        machine.set_mem(2, verb);
        machine.next();
        machine.mem(0)
    };

    // Part 1: just run with a given noun/verb combination.
    advtools::verify("Restored state", run_with(12, 2), 3895705);

    // Part 2: try different nouns/verbs to get the desired landing date.
    let (noun, verb) = (0..100i64).into_par_iter().find_map_first(|noun| {
        (0..100).find(|&verb| run_with(noun, verb) == LANDING)
                .map(|verb| (noun, verb))
    }).unwrap();
    advtools::verify("Correct noun/verb", 100*noun + verb, 6417);
}
