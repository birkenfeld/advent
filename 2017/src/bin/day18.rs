extern crate advtools;
use advtools::prelude::*;
use std::collections::VecDeque;

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

fn reg(s: &str) -> usize {
    (s.chars().item() as u8 - b'a') as usize
}

fn reg_or_imm(s: &str) -> Arg {
    s.parse().ok().map_or_else(|| Arg::Reg(reg(s)), Arg::Imm)
}

struct Machine<'a> {
    prog: &'a [Op],
    regs: [i64; 16],
    pc:   usize,
    snd:  VecDeque<i64>,
    nsnd: usize,
}

impl<'a> Machine<'a> {
    fn new(prog: &[Op]) -> Machine {
        Machine { prog: prog, regs: [0; 16], pc: 0, snd: VecDeque::new(), nsnd: 0 }
    }

    fn get(&self, arg: Arg) -> i64 {
        match arg {
            Arg::Imm(i) => i,
            Arg::Reg(n) => self.regs[n],
        }
    }

    fn run(&mut self, rcv: &mut VecDeque<i64>) -> bool {
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

    let mut m = Machine::new(&program);
    m.run(&mut VecDeque::new());
    println!("Recovered: {}", m.snd.pop_back().unwrap());

    let mut m0 = Machine::new(&program);
    let mut m1 = Machine::new(&program);
    m1.regs[15] = 1; // machine ID
    loop {
        if !m0.run(&mut m1.snd) && !m1.run(&mut m0.snd) {
            break;
        }
    }
    println!("Program 1 send count: {}", m1.nsnd);
}
