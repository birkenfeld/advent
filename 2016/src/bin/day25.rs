use advtools::prelude::HashMap;
use advtools::input::iter_input;

type RegNo = usize;

#[derive(Clone, Copy, Debug)]
enum Arg {
    Imm(i64),
    Reg(RegNo),
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Out(RegNo),
    Inc(RegNo),
    Dec(RegNo),
    Cpy(Arg, Arg),
    Jnz(Arg, Arg),
}

fn reg(s: &str) -> usize {
    match s {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("unknown register: {}", s)
    }
}

fn reg_or_imm(s: &str) -> Arg {
    s.parse().ok().map_or_else(|| Arg::Reg(reg(s)), Arg::Imm)
}

#[derive(Default)]
struct Machine {
    regs: [i64; 4],
    prog: Vec<Op>,
    pc:   usize,
    map:  HashMap<(usize, [i64; 4]), Vec<i64>>,
    sig:  Vec<i64>,
}

impl Machine {
    fn new(prog: Vec<Op>) -> Machine {
        Machine { prog, .. Machine::default() }
    }
    fn get(&self, arg: Arg) -> i64 {
        match arg {
            Arg::Imm(i) => i,
            Arg::Reg(n) => self.regs[n],
        }
    }
    fn out(&mut self, reg: RegNo) -> bool {
        let i = self.regs[reg];
        let ok = (i == 0 || i == 1) && i != *self.sig.last().unwrap_or(&1);
        self.sig.push(i);
        ok
    }
    fn run(&mut self) -> bool {
        loop {
            let op = self.prog[self.pc];
            self.pc += 1;
            match op {
                Op::Out(reg) => {
                    if self.map.contains_key(&(self.pc, self.regs)) {
                        return true;
                    }
                    self.map.insert((self.pc, self.regs), self.sig.clone());
                    if !self.out(reg) { return false; }
                },
                Op::Inc(reg) => self.regs[reg] += 1,
                Op::Dec(reg) => self.regs[reg] -= 1,
                Op::Cpy(src, dst) => if let Arg::Reg(reg) = dst {
                     self.regs[reg] = self.get(src)
                },
                Op::Jnz(tst, ofs) => if self.get(tst) != 0 {
                    self.pc = ((self.pc as isize - 1) + self.get(ofs) as isize) as usize
                },
            }
        }
    }
    fn reset(&mut self) {
        self.regs = [0, 0, 0, 0];
        self.pc = 0;
        self.map.clear();
        self.sig.clear();
    }
}

fn main() {
    let mut program = Vec::new();
    for line in iter_input::<Vec<String>>() {
        program.push(
            match &*line[0] {
                "out" => Op::Out(reg(&line[1])),
                "inc" => Op::Inc(reg(&line[1])),
                "dec" => Op::Dec(reg(&line[1])),
                "cpy" => Op::Cpy(reg_or_imm(&line[1]), reg_or_imm(&line[2])),
                "jnz" => Op::Jnz(reg_or_imm(&line[1]), reg_or_imm(&line[2])),
                _ => panic!("unknown op: {}", &line[0])
            }
        );
    }

    let mut machine = Machine::new(program);
    for i in 0.. {
        machine.reset();
        machine.regs[0] = i;
        if machine.run() {
            advtools::print("Register a to generate clock", i);
            break;
        }
    }
}
