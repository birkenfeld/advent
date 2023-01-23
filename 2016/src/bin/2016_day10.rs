use advtools::prelude::{HashMap, Itertools};
use advtools::input;

const FORMAT: &str = r"(?:bot (\d+) .* to (\S+) (\d+) .* to (\S+) (\d+)|value (\d+)\D+(\d+))";

enum Tgt {
    Out(u32),
    Bot(u32),
}

struct Bot {
    chips: Vec<u32>,
    targets: (Tgt, Tgt),
}

fn main() {
    let mut bots = HashMap::<u32, Bot>::new();
    let mut outputs = HashMap::<u32, u32>::new();
    let mut changes = Vec::new();

    for (botrule, valrule) in input::rx_lines::<(Option<_>, Option<_>)>(FORMAT) {
        if let Some((val, botno)) = valrule {
            changes.push((botno, val));
        } else if let Some((botno, lowtgt, low, hightgt, high)) = botrule {
            bots.insert(botno, Bot { chips: vec![], targets: (
                if matches!(lowtgt, "output") { Tgt::Out(low) } else { Tgt::Bot(low) },
                if matches!(hightgt, "output") { Tgt::Out(high) } else { Tgt::Bot(high) }
            ) });
        }
    }

    while !changes.is_empty() {
        for (botno, chip) in changes.drain(..) {
            bots.get_mut(&botno).unwrap().chips.push(chip);
        }
        for (botno, bot) in &mut bots {
            if bot.chips.len() == 2 {
                let chips = bot.chips.drain(..).sorted().collect_vec();
                if chips == [17, 61] {
                    advtools::verify("Comparing 17-61", botno, 101);
                }
                match bot.targets.0 {
                    Tgt::Out(low) => { outputs.insert(low, chips[0]); }
                    Tgt::Bot(low) => { changes.push((low, chips[0])); }
                }
                match bot.targets.1 {
                    Tgt::Out(high) => { outputs.insert(high, chips[1]); }
                    Tgt::Bot(high) => { changes.push((high, chips[1])); }
                }
            }
        }
    }

    advtools::verify("Outputs 0*1*2", outputs[&0] * outputs[&1] * outputs[&2], 37789);
}
