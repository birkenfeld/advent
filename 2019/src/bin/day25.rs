use advtools::prelude::{Itertools, HashMap};
use advtools::input;
use advent19::{Machine, IO};

fn read_out(machine: &mut Machine) -> Vec<String> {
    let mut out = String::new();
    let mut lines = Vec::new();
    loop {
        match machine.run() {
            IO::Output(n) => match n as u8 as char {
                '\n' => lines.push(std::mem::take(&mut out)),
                ch => out.push(ch),
            }
            IO::Input | IO::Halt => return lines,
        }
    }
}

fn inverse(dir: &str) -> &str {
    match dir {
        "north\n" => "south\n",
        "south\n" => "north\n",
        "east\n" => "west\n",
        "west\n" => "east\n",
        _ => unreachable!()
    }
}

fn main() {
    let code = Machine::parse(input::string());

    // Step 1: Find all the rooms, items and directions.
    let mut rooms = HashMap::new();
    let mut queue = vec![(vec![], Machine::new(&code))];

    loop {
        for (steps, mut mach) in std::mem::take(&mut queue) {
            let steps: Vec<String> = steps;
            let mut name = String::new();
            let mut doors = vec![];
            let mut items = vec![];
            let mut iter = read_out(&mut mach).into_iter();
            while let Some(line) = iter.next() {
                if line.starts_with("==") {
                    name = line[3..line.len() - 3].into();
                } else if line == "Doors here lead:" {
                    for line in iter.by_ref() {
                        if let Some(rest ) = line.strip_prefix("- ") {
                            doors.push(format!("{}\n", rest));
                        } else {
                            break;
                        }
                    }
                } else if line == "Items here:" {
                    for line in iter.by_ref() {
                        if let Some(rest) = line.strip_prefix("- ") {
                            items.push(rest.to_string());
                        } else {
                            break;
                        }
                    }
                }
            }
            if rooms.insert(name, (steps.clone(), items)).is_some() {
                continue;
            }

            for dir in doors {
                if steps.last().map(|v| &**v) == Some(inverse(&dir)) {
                    continue;
                }
                let new_mach = mach.clone().with_input_str(&dir);
                let mut new_steps = steps.clone();
                new_steps.push(dir);
                queue.push((new_steps, new_mach));
            }
        }
        if queue.is_empty() {
            break;
        }
    }

    // Step 2: Take all the items.
    let mut my_items = Vec::new();
    let mut machine = Machine::new(&code);
    for (_, (steps, items)) in &rooms {
        if items.is_empty() || ["giant electromagnet", "infinite loop", "photons",
                                "escape pod", "molten lava"].contains(&&*items[0])
        {
            continue;
        }
        for dir in steps {
            machine = machine.with_input_str(dir);
        }
        machine = machine.with_input_str(&format!("take {}\n", items[0]));
        my_items.push(items[0].clone());
        for dir in steps.iter().rev() {
            machine = machine.with_input_str(inverse(dir));
        }
    }

    // Step 3: Go to the checkpoint and drop everything.
    for dir in &rooms["Security Checkpoint"].0 {
        machine = machine.with_input_str(dir);
    }
    for item in &my_items {
        machine = machine.with_input_str(&format!("drop {}\n", item));
    }
    read_out(&mut machine);  // let the machine run until here

    // Step 4: Try all combinations of items.
    for i in 1..my_items.len() {
        for comb in my_items.iter().combinations(i) {
            for item in &comb {
                machine = machine.with_input_str(&format!("take {}\n", item));
            }
            machine = machine.with_input_str("north\n");
            for line in read_out(&mut machine) {
                if line.contains("keypad") {
                    advtools::verify("Airlock password",
                                    line.split_whitespace().nth(11).unwrap(), 35717128);
                    return;
                }
            }
            for item in comb {
                machine = machine.with_input_str(&format!("drop {}\n", item));
            }
        }
    }

}
