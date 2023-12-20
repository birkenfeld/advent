use advtools::input;
use advtools::prelude::{Uids, HashMap, Itertools, lcm};

type Mod = usize;

enum Module {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<Mod, bool>),
}

// Run pulses from a single button press.
fn press(iter: usize, modules: &mut HashMap<Mod, (Vec<Mod>, Module)>, pre_output: Mod,
         cycles: &mut HashMap<Mod, usize>) -> (usize, usize) {
    let mut pulses = (0, 0);
    let mut queue = vec![(0, 0, false)];
    while !queue.is_empty() {
        for (source, target, high) in std::mem::take(&mut queue) {
            if high {
                pulses.1 += 1;
            } else {
                pulses.0 += 1;
            }
            match modules.get_mut(&target) {
                Some((outputs, Module::FlipFlop(on))) => if !high {
                    *on = !*on;
                    for output in outputs {
                        queue.push((target, *output, *on));
                    }
                }
                Some((outputs, Module::Conjunction(inputs))) => {
                    // If the conjunction before the "rx" output gets a high
                    // pulse from one of its inputs, note the cycle number.
                    if target == pre_output && high {
                        let _ = cycles.try_insert(source, iter);
                    }
                    inputs.insert(source, high);
                    let all_high = inputs.values().all(|&v| v);
                    for output in outputs {
                        queue.push((target, *output, !all_high));
                    }
                }
                Some((outputs, Module::Broadcast)) => for output in outputs {
                    queue.push((target, *output, high));
                }
                _ => ()
            }
        }
    }
    pulses
}

fn main() {
    // This solution assumes the basic structure of the node graph to be as follows:
    //
    // - There are subgraphs that deliver a high pulse after each cycle of a
    // certain number of presses.
    //
    // - There is one Conjunction module that collects a signal from each
    // subgraph and outputs to the final node "rx", we need to remember that
    // to determine each subgraph's cycle length.
    //
    // - The final iteration when the "rx" node gets its low signal is the LCM of
    // all subgraph cycle lengths.
    let mut pre_output = 0;

    // Parse the input. First stage, collect modules and their inputs and outputs.
    let mut modules = HashMap::new();
    let mut inputs = HashMap::<Mod, HashMap<Mod, bool>>::new();
    let mut mod_ids = Uids::new();
    mod_ids.get_id("broadcaster");  // Ensure broadcaster has ID 0
    for (typ, name, outputs) in input::rx_lines::<(&str, &str, &str)>("([%&])?(.*) -> (.*)") {
        let name = mod_ids.get_id(name);
        if outputs == "rx" {
            pre_output = name;
        }
        let outputs = outputs.split(", ").map(|v| mod_ids.get_id(v)).collect_vec();
        for &output in &outputs {
            inputs.entry(output).or_default().insert(name, false);
        }
        let module = match typ {
            "%" => (outputs, Module::FlipFlop(false)),
            "&" => (outputs, Module::Conjunction(HashMap::new())),
            _   => (outputs, Module::Broadcast),
        };
        modules.insert(name, module);
    }

    // Second stage: assign the map of inputs to the Conjunction modules.
    let mut n_cycles = 0;
    for (name, inputs) in inputs {
        match &mut modules.get_mut(&name) {
            Some((_, Module::Conjunction(map))) => {
                if name == pre_output {
                    n_cycles = inputs.len();
                }
                *map = inputs;
            }
            _ => ()
        }
    }

    // Run button presses until we've found the cycle length for each subdivision
    // of the machine.
    let mut cycles = HashMap::new();
    let mut pulses = (0, 0);
    for iter in 1.. {
        let res = press(iter, &mut modules, pre_output, &mut cycles);
        pulses.0 += res.0;
        pulses.1 += res.1;
        if iter == 1000 {
            advtools::verify("Low*high after 1000 presses", pulses.0 * pulses.1, 839775244);
        }
        if cycles.len() == n_cycles {
            let min_presses = cycles.into_values().reduce(lcm).unwrap();
            advtools::verify("Presses for low to rx", min_presses, 207787533680413_u64);
            break;
        }
    }
}
