use advtools::prelude::Itertools;
use advtools::input::input_string;
use advent19::Machine;

fn main() {
    let cells = input_string().trim().split(',').map(|v| v.parse().unwrap()).collect_vec();

    let max_signal = permutohedron::Heap::new(&mut [0, 1, 2, 3, 4]).map(|phases| {
        phases.iter().fold(0, |signal, &phase| {
            Machine::new(&cells, vec![phase, signal]).next().unwrap()
        })
    }).max();

    advtools::print("First round", max_signal.unwrap());

    let max_signal = permutohedron::Heap::new(&mut [5, 6, 7, 8, 9]).map(|phases| {
        let mut signal = 0;
        let mut machines = phases.iter().map(|&ph| Machine::new(&cells, Some(ph))).collect_vec();
        loop {
            for machine in &mut machines {
                match machine.run(Some(signal)) {
                    Some(new) => signal = new,
                    None => return signal
                }
            }
        }
    }).max();

    advtools::print("Second round", max_signal.unwrap());
}
