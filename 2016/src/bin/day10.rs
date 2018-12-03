extern crate advtools;
use advtools::prelude::{HashMap, Itertools};
use advtools::input::{iter_input, parse_parts};

enum Rule {
    Out(u32),
    Bot(u32),
}

struct Bot {
    chips: Vec<u32>,
    rule: (Rule, Rule),
}

fn main() {
    let mut bots = HashMap::<u32, Bot>::default();
    let mut outputs = HashMap::<u32, u32>::default();
    let mut changes = Vec::new();

    for line in iter_input::<String>() {
        if line.starts_with("value") {
            let (val, botno) = parse_parts(&line, (1, 5));
            changes.push((botno, val));
        } else {
            let (botno, lowrule, low, highrule, high): (u32, String, u32, String, u32) =
                parse_parts(&line, (1, 5, 6, 10, 11));
            let lowrule = if lowrule == "output" { Rule::Out(low) } else { Rule::Bot(low) };
            let highrule = if highrule == "output" { Rule::Out(high) } else { Rule::Bot(high) };
            bots.insert(botno, Bot { chips: vec![], rule: (lowrule, highrule) });
        }
    }

    while !changes.is_empty() {
        for (botno, chip) in changes.drain(..) {
            bots.get_mut(&botno).unwrap().chips.push(chip);
        }
        for (botno, bot) in &mut bots {
            if bot.chips.len() == 2 {
                let chips = bot.chips.drain(..).sorted();
                if chips == [17, 61] {
                    println!("Comparing 17-61: {}", botno);
                }
                match bot.rule.0 {
                    Rule::Out(low) => { outputs.insert(low, chips[0]); }
                    Rule::Bot(low) => { changes.push((low, chips[0])); }
                }
                match bot.rule.1 {
                    Rule::Out(high) => { outputs.insert(high, chips[1]); }
                    Rule::Bot(high) => { changes.push((high, chips[1])); }
                }
            }
        }
    }

    println!("Outputs 0*1*2: {}", outputs[&0] * outputs[&1] * outputs[&2]);
}
