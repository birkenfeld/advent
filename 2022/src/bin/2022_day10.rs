use advtools::input;
use advtools::prelude::Itertools;

const INSTR: &str = r"addx (-?\d+)|noop";

#[derive(Default)]
struct Machine {
    out: String,
    cycle: i32,
    reg_x: i32,
    strengths: i32,
}

impl Machine {
    fn one_cycle(&mut self) {
        self.cycle += 1;
        if self.cycle % 40 == 1 {
            self.out.push('\n');
        }

        // Part 1: record "strengths" in the middle of scanlines.
        if self.cycle % 40 == 20 {
            self.strengths += self.cycle * self.reg_x;
        }

        // Part 2: check if the current pixel is lit or not.
        let pix = (self.cycle - 1) % 40;
        self.out.push(if pix >= self.reg_x - 1 && pix <= self.reg_x + 1 { '█' } else { ' ' });
    }

    fn run(&mut self, ops: Vec<Option<i32>>) {
        for op in ops {
            self.one_cycle();
            if let Some(i) = op {
                self.one_cycle();
                // The register is only updated after both cycles complete.
                self.reg_x += i;
            }
        }
    }
}

fn main() {
    let mut machine = Machine { reg_x: 1, .. Machine::default() };
    // Read input. Some(x) = addx, None = noop.
    let ops = input::rx_lines::<Option<i32>>(INSTR).collect_vec();
    machine.run(ops);

    advtools::verify("Strength sum", machine.strengths, 14520);
    advtools::print("Message", machine.out);
}
