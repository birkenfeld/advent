use advtools::input;
use advtools::prelude::{Itertools, VecDeque};

const RX: &str = r"move (\d+) from (\d+) to (\d+)|(.*)";

fn run(instrs: &[(usize, usize, usize)], mut stacks: Vec<VecDeque<char>>, multi: bool) -> String {
    for &(n, from, to) in instrs {
        let m = stacks[from-1].len();
        let transfer = stacks[from-1].drain(m-n..).collect_vec();
        if multi {
            stacks[to-1].extend(transfer);
        } else {
            stacks[to-1].extend(transfer.iter().rev());
        }
    }
    stacks.iter().map(|s| s[s.len()-1]).collect()
}

fn main() {
    let mut instrs = vec![];
    let mut stacks = (0..9).map(|_| VecDeque::new()).collect_vec();
    for (instr, line) in input::rx_lines::<(Option<(usize, usize, usize)>, &str)>(RX) {
        if let Some(instr) = instr {
            instrs.push(instr);
        } else if line.contains('[') {
            line.chars().skip(1).step_by(4).enumerate().for_each(|(i, c)| {
                if c != ' ' {
                    stacks[i].push_front(c);
                }
            });
        }
    }

    advtools::verify("CrateMover 9000", run(&instrs, stacks.clone(), false), "LBLVVTVLP");
    advtools::verify("CrateMover 9001", run(&instrs, stacks, true), "TPFFBDRJD");
}
