use advtools::input::iter_input;

type RegNo = usize;

#[derive(Clone, Copy, Debug)]
enum Arg {
    Imm(i64),
    Reg(RegNo),
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Tgl(RegNo),
    Inc(RegNo),
    Dec(RegNo),
    Cpy(Arg, Arg),
    Jnz(Arg, Arg),
    Nop,
    Add(RegNo, RegNo),
    Mul(RegNo, Arg, RegNo, RegNo),
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

struct Machine {
    regs: [i64; 4],
    prog: Vec<Op>,
    pc:   usize,
}

impl Machine {
    fn new(mut prog: Vec<Op>) -> Machine {
        Self::opt(&mut prog);
        Machine { prog: prog, regs: [0, 0, 0, 0], pc: 0 }
    }
    fn opt(prog: &mut Vec<Op>) {
        for i in 0..prog.len() - 2 {
            if let Op::Inc(r1) = prog[i] {
                if let Op::Dec(r2) = prog[i+1] {
                    if let Op::Jnz(Arg::Reg(r3), Arg::Imm(-2)) = prog[i+2] {
                        if r2 == r3 {
                            prog[i] = Op::Add(r1, r2);
                            prog[i+1] = Op::Nop;
                            prog[i+2] = Op::Nop;
                        }
                    }
                }
            }
            if i+5 < prog.len() {
                if let Op::Cpy(mul1, Arg::Reg(r1)) = prog[i] {
                    if let Op::Add(r2, r3) = prog[i+1] {
                        if let Op::Dec(r4) = prog[i+4] {
                            if let Op::Jnz(Arg::Reg(r5), Arg::Imm(-5)) = prog[i+5] {
                                if r1 == r3 && r4 == r5 {
                                    prog[i] = Op::Mul(r2, mul1, r4, r1);
                                    prog[i+1] = Op::Nop;
                                    prog[i+4] = Op::Nop;
                                    prog[i+5] = Op::Nop;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn get(&self, arg: Arg) -> i64 {
        match arg {
            Arg::Imm(i) => i,
            Arg::Reg(n) => self.regs[n],
        }
    }
    fn run(&mut self) {
        while self.pc < self.prog.len() {
            let op = self.prog[self.pc];
            self.pc += 1;
            match op {
                Op::Tgl(reg) => {
                    let pos = (self.pc as i64 - 1 + self.regs[reg]) as usize;
                    if pos < self.prog.len() {
                        self.prog[pos] = match self.prog[pos] {
                            Op::Tgl(reg) |
                            Op::Dec(reg) => Op::Inc(reg),
                            Op::Inc(reg) => Op::Dec(reg),
                            Op::Cpy(v1, v2) => Op::Jnz(v1, v2),
                            Op::Jnz(v1, v2) => Op::Cpy(v1, v2),
                            op => panic!("cannot toggle: {:?}", op)
                        };
                        Self::opt(&mut self.prog);
                    }
                }
                Op::Inc(reg) => self.regs[reg] += 1,
                Op::Dec(reg) => self.regs[reg] -= 1,
                Op::Cpy(src, dst) => if let Arg::Reg(reg) = dst {
                     self.regs[reg] = self.get(src)
                },
                Op::Jnz(tst, ofs) => if self.get(tst) != 0 {
                    self.pc = ((self.pc as isize - 1) + self.get(ofs) as isize) as usize
                },
                Op::Nop => {},
                Op::Add(dst, src) => {
                    self.regs[dst] += self.regs[src];
                    self.regs[src] = 0;
                },
                Op::Mul(dst, src1, src2, hlp) => {
                    self.regs[dst] += self.get(src1) * self.regs[src2];
                    self.regs[hlp] = 0;
                },
            }
        }
    }
}

fn main() {
    let mut program = Vec::new();
    for line in iter_input::<Vec<String>>() {
        program.push(
            match &*line[0] {
                "tgl" => Op::Tgl(reg(&line[1])),
                "inc" => Op::Inc(reg(&line[1])),
                "dec" => Op::Dec(reg(&line[1])),
                "cpy" => Op::Cpy(reg_or_imm(&line[1]),
                                 reg_or_imm(&line[2])),
                "jnz" => Op::Jnz(reg_or_imm(&line[1]),
                                 reg_or_imm(&line[2])),
                _ => panic!("unknown op: {}", &line[0])
            }
        );
    }

    let mut machine = Machine::new(program.clone());
    machine.regs[0] = 7;
    machine.run();
    advtools::print("a (7 eggs)", machine.regs[0]);
    let mut machine = Machine::new(program);
    machine.regs[0] = 12;
    machine.run();
    advtools::print("a (12 eggs)", machine.regs[0]);
}
