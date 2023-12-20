use advtools::input;
use advtools::prelude::{Uids, HashMap, Itertools};

type Mod = usize;

enum Module {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<Mod, bool>),
}

// Run a single pulse from the button.
fn pulse(i: usize, modules: &mut HashMap<Mod, (Vec<Mod>, Module)>, pre_output: Mod,
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
                    if target == pre_output && high {
                        let _ = cycles.try_insert(source, i);
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
    // There is one Conjunction module that outputs to the final node "rx", we need
    // to remember that because it's important for part 2.
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

    let mut cycles = HashMap::new();
    let mut pulses = (0, 0);
    for i in 1.. {
        let res = pulse(i, &mut modules, pre_output, &mut cycles);
        pulses.0 += res.0;
        pulses.1 += res.1;
        if i == 1000 {
            advtools::verify("Low*high after 1000 presses", pulses.0 * pulses.1, 839775244);
        }
        if cycles.len() == n_cycles {
            advtools::verify("Presses for low to rx",
                             cycles.values().product::<usize>(), 207787533680413_u64);
            break;
        }
    }
}
