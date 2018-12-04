extern crate advtools;
use advtools::prelude::{Itertools, VecDeque};
use advtools::input::iter_input;

type RegNo = usize;

#[derive(Clone, Copy, Debug)]
enum Arg {
    Imm(i64),
    Reg(RegNo),
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Snd(Arg),
    Rcv(RegNo),
    Set(RegNo, Arg),
    Add(RegNo, Arg),
    Mul(RegNo, Arg),
    Mod(RegNo, Arg),
    Jgz(Arg, Arg),
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
struct Machine<'a> {
    prog: &'a [Op],
    regs: [i64; 16],
    pc:   usize,
    snd:  VecDeque<i64>,
    nsnd: usize,
}

impl<'a> Machine<'a> {
    fn new(prog: &[Op], id: i64) -> Machine {
        let mut m = Machine { prog: prog, regs: [0; 16], pc: 0, snd: VecDeque::new(), nsnd: 0 };
        m.regs[15] = id;
        m
    }

    /// Get value represented by operand (register or immediate).
    fn get(&self, arg: Arg) -> i64 {
        match arg {
            Arg::Imm(i) => i,
            Arg::Reg(n) => self.regs[n],
        }
    }

    /// Execute instructions until the next receive instruction that can't be
    /// fulfilled from the given queue.
    fn run(&mut self, rcv: &mut VecDeque<i64>) -> bool {
        // To determine the deadlock condition, we keep track of whether we've
        // advanced by at least one instruction in this run.  When both machines
        // haven't advanced, there is a deadlock.
        let mut advanced = false;
        loop {
            match self.prog[self.pc] {
                Op::Snd(arg) => {
                    let val = self.get(arg);
                    self.snd.push_back(val);
                    self.nsnd += 1;
                },
                Op::Rcv(reg) => {
                    if let Some(val) = rcv.pop_front() {
                        self.regs[reg] = val;
                    } else {
                        return advanced;
                    }
                },
                Op::Set(reg, arg) => self.regs[reg] = self.get(arg),
                Op::Add(reg, arg) => self.regs[reg] += self.get(arg),
                Op::Mul(reg, arg) => self.regs[reg] *= self.get(arg),
                Op::Mod(reg, arg) => self.regs[reg] %= self.get(arg),
                Op::Jgz(cond, offset) => if self.get(cond) > 0 {
                    self.pc = (self.pc as i64 + self.get(offset) - 1) as usize;
                }
            }
            self.pc += 1;
            advanced = true;
        }
    }
}

fn main() {
    let program = iter_input::<Vec<String>>().map(|line| match &*line[0] {
        "snd" => Op::Snd(reg_or_imm(&line[1])),
        "rcv" => Op::Rcv(reg(&line[1])),
        "set" => Op::Set(reg(&line[1]), reg_or_imm(&line[2])),
        "add" => Op::Add(reg(&line[1]), reg_or_imm(&line[2])),
        "mul" => Op::Mul(reg(&line[1]), reg_or_imm(&line[2])),
        "mod" => Op::Mod(reg(&line[1]), reg_or_imm(&line[2])),
        "jgz" => Op::Jgz(reg_or_imm(&line[1]), reg_or_imm(&line[2])),
        _ => panic!("unknown op: {}", line[0])
    }).collect_vec();

    // Part 1: Run one machine, without any receive queue, and determine the
    // last sent value.
    let mut m = Machine::new(&program, 0);
    m.run(&mut VecDeque::new());
    println!("Recovered: {}", m.snd.pop_back().unwrap());

    // Part 2: Run two machines.  For the `while` condition see above.
    let mut m0 = Machine::new(&program, 0);
    let mut m1 = Machine::new(&program, 1);
    while m0.run(&mut m1.snd) || m1.run(&mut m0.snd) { }
    println!("Program 1 send count: {}", m1.nsnd);
}
