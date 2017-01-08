extern crate advtools;

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
    fn new(prog: Vec<Op>) -> Machine {
        Machine { prog: prog, regs: [0, 0, 0, 0], pc: 0 }
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
                            Op::Tgl(reg) => Op::Inc(reg),
                            Op::Inc(reg) => Op::Dec(reg),
                            Op::Dec(reg) => Op::Inc(reg),
                            Op::Cpy(v1, v2) => Op::Jnz(v1, v2),
                            Op::Jnz(v1, v2) => Op::Cpy(v1, v2),
                        }
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
            }
        }
    }
}

fn main() {
    let mut program = Vec::new();
    for line in advtools::iter_input::<Vec<String>>() {
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
    println!("a (7 eggs): {}", machine.regs[0]);
    let mut machine = Machine::new(program);
    machine.regs[0] = 12;
    machine.run();
    println!("a (12 eggs): {}", machine.regs[0]);
}
