extern crate advtools;

use std::collections::HashMap;

fn main() {
    let mut rules = HashMap::<u32, (bool, u32, bool, u32)>::new();
    let mut values = HashMap::<u32, Vec<u32>>::new();
    let mut outputs = HashMap::<u32, u32>::new();
    let mut circulating = 0;

    for line in advtools::iter_input::<String>() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts[0] == "value" {
            let val = parts[1].parse().unwrap();
            let bot = parts[5].parse().unwrap();
            values.entry(bot).or_insert(vec![]).push(val);
            circulating += 1;
        } else {
            let bot = parts[1].parse().unwrap();
            let lowout = parts[5] == "output";
            let low = parts[6].parse().unwrap();
            let highout = parts[10] == "output";
            let high = parts[11].parse().unwrap();
            rules.insert(bot, (lowout, low, highout, high));
        }
    }

    let mut changes = Vec::new();
    while circulating > 0 {
        for (bot, chips) in &mut values {
            if chips.len() == 2 {
                let (lowout, low, highout, high) = rules[bot];
                let mut chips = std::mem::replace(chips, vec![]);
                chips.sort();
                if &chips == &[17, 61] {
                    println!("Comparing 17-61: {}", bot);
                }
                if lowout {
                    outputs.insert(low, chips[0]);
                    circulating -= 1;
                } else {
                    changes.push((low, chips[0]));
                }
                if highout {
                    outputs.insert(high, chips[1]);
                    circulating -= 1;
                } else {
                    changes.push((high, chips[1]));
                }
            }
        }
        for (bot, chip) in changes.drain(..) {
            values.entry(bot).or_insert(vec![]).push(chip);
        }
    }

    println!("Outputs 0*1*2: {}", outputs[&0] * outputs[&1] * outputs[&2]);
}
