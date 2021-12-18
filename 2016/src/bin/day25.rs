use advtools::prelude::HashMap;
use advtools::input;
use advent16::Machine;

fn main() {
    let mut machine = Machine::new(input::parse_lines());

    'outer: for i in 0.. {
        let mut sig = Vec::new();
        let mut map = HashMap::new();
        machine.reset();
        machine.set_reg(0, i);
        while let Some(j) = machine.run() {
            if map.contains_key(&machine.get_state()) {
                advtools::verify("Register a to generate clock", i, 182);
                return;
            }
            map.insert(machine.get_state(), sig.clone());

            let ok = (j == 0 || j == 1) && j != *sig.last().unwrap_or(&1);
            sig.push(j);
            if !ok {
                continue 'outer;
            }
        }
    }
}
