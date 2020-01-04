use advtools::prelude::Itertools;
use advtools::input::iter_input;

type RegNo = usize;

#[derive(Clone, Copy, Debug)]
enum Arg {
    Imm(i64),
    Reg(RegNo),
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Set(RegNo, Arg),
    Sub(RegNo, Arg),
    Mul(RegNo, Arg),
    Jnz(Arg, Arg),
}

/// Parse register.
fn reg(s: &str) -> usize {
    (s.chars().next().unwrap() as u8 - b'a') as usize
}

/// Parse register or immediate operand.
fn reg_or_imm(s: &str) -> Arg {
    s.parse().ok().map_or_else(|| Arg::Reg(reg(s)), Arg::Imm)
}

/// Represents one copy of the program.  The queue is used to store sent values.
struct Machine {
    prog: Vec<Op>,
    regs: [i64; 8],
    pc:   usize,
    nmul: usize,
}

impl Machine {
    fn new(prog: Vec<Op>) -> Machine {
        Machine { prog: prog, regs: [0; 8], pc: 0, nmul: 0 }
    }

    /// Get value represented by operand (register or immediate).
    fn get(&self, arg: Arg) -> i64 {
        match arg {
            Arg::Imm(i) => i,
            Arg::Reg(n) => self.regs[n],
        }
    }

    fn run(&mut self) {
        loop {
            match self.prog[self.pc] {
                Op::Set(reg, arg) => self.regs[reg] = self.get(arg),
                Op::Sub(reg, arg) => self.regs[reg] -= self.get(arg),
                Op::Mul(reg, arg) => {
                    self.regs[reg] *= self.get(arg);
                    self.nmul += 1;
                }
                Op::Jnz(cond, offset) => if self.get(cond) != 0 {
                    self.pc = (self.pc as i64 + self.get(offset) - 1) as usize;
                }
            }
            self.pc += 1;
            if self.pc >= self.prog.len() {
                break;
            }
        }
    }
}

fn main() {
    let program = iter_input::<Vec<String>>().map(|line| match &*line[0] {
        "set" => Op::Set(reg(&line[1]), reg_or_imm(&line[2])),
        "sub" => Op::Sub(reg(&line[1]), reg_or_imm(&line[2])),
        "mul" => Op::Mul(reg(&line[1]), reg_or_imm(&line[2])),
        "jnz" => Op::Jnz(reg_or_imm(&line[1]), reg_or_imm(&line[2])),
        _ => panic!("unknown op: {}", line[0])
    }).collect_vec();

    // Part 1: Run machine.
    let mut m = Machine::new(program.clone());
    m.run();
    advtools::verify("Number of `mul`s", m.nmul, 6724);

    // Part 2: Run the algorithm, translated:
    // Determine number of non-primes between b and c, inclusive, in steps of 17.
    // b = 108400, c = 125400.
    let mut nonprimes = 0;
    'outer: for n in (108400..=125400).step_by(17) {
        for i in 2..=(n as f64).sqrt() as i64 {
            if n % i == 0 {
                nonprimes += 1;
                continue 'outer;
            }
        }
    }
    advtools::verify("Nonprimes (`h` register)", nonprimes, 903);
}
