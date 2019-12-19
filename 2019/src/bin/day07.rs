use advtools::prelude::Itertools;
use advtools::input::input_string;
use advent19::Machine;

fn main() {
    let code = Machine::parse(&input_string());

    // Find the maximum signal by testing all permutations of 0..5.
    let max_signal = (0..5).permutations(5).map(|phases| {
        // This fold applies the signal to all five machines in order.
        phases.iter().fold(0, |signal, &phase| {
            Machine::new(&code).run(vec![phase, signal]).unwrap()
        })
    }).max();

    advtools::print("Max signal", max_signal.unwrap());

    // Same spiel as in part 1, but a bit more complex since we have
    // to loop until a machine halts.
    let max_signal = (5..10).permutations(5).map(|phases| {
        let mut signal = 0;
        let mut machines =
            phases.iter().map(|&ph| Machine::new(&code).with_input(Some(ph))).collect_vec();
        loop {
            for machine in &mut machines {
                match machine.run(Some(signal)) {
                    Some(new) => signal = new,
                    None => return signal
                }
            }
        }
    }).max();

    advtools::print("Max signal with feedback", max_signal.unwrap());
}
