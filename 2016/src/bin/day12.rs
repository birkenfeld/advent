use advtools::input::iter_input;

type RegNo = usize;

#[derive(Clone, Copy, Debug)]
enum Op {
    CpyReg(RegNo, RegNo),
    CpyConst(i64, RegNo),
    Inc(RegNo),
    Dec(RegNo),
    Jnz(RegNo, isize),
    Jmp(isize),
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

struct Machine {
    regs: [i64; 4],
    prog: Vec<Op>,
    pc:   usize,
}

impl Machine {
    fn new(prog: Vec<Op>) -> Machine {
        Machine { prog: prog, regs: [0, 0, 0, 0], pc: 0 }
    }
    fn run(&mut self) {
        while self.pc < self.prog.len() {
            let op = self.prog[self.pc];
            self.pc += 1;
            match op {
                Op::Inc(reg) => self.regs[reg] += 1,
                Op::Dec(reg) => self.regs[reg] -= 1,
                Op::CpyConst(val, reg) => self.regs[reg] = val,
                Op::CpyReg(reg1, reg2) => self.regs[reg2] = self.regs[reg1],
                Op::Jmp(ofs) => self.pc = ((self.pc as isize - 1) + ofs) as usize,
                Op::Jnz(reg, ofs) => if self.regs[reg] != 0 {
                    self.pc = ((self.pc as isize - 1) + ofs) as usize
                }
            }
        }
    }
    fn reset(&mut self) {
        self.pc = 0;
        self.regs = [0, 0, 0, 0];
    }
}

fn main() {
    let mut program = Vec::new();
    for line in iter_input::<Vec<String>>() {
        program.push(
            match &*line[0] {
                "inc" => Op::Inc(reg(&line[1])),
                "dec" => Op::Dec(reg(&line[1])),
                "cpy" => match line[1].parse() {
                    Ok(cst) => Op::CpyConst(cst, reg(&line[2])),
                    Err(_)  => Op::CpyReg(reg(&line[1]), reg(&line[2]))
                },
                "jnz" => match line[1].parse::<isize>() {
                    Ok(_)  => Op::Jmp(line[2].parse().unwrap()),
                    Err(_) => Op::Jnz(reg(&line[1]), line[2].parse().unwrap()),
                },
                _ => panic!("unknown op: {}", &line[0])
            }
        );
    }

    let mut machine = Machine::new(program);
    machine.run();
    println!("Register a for c = 0: {}", machine.regs[0]);
    machine.reset();
    machine.regs[2] = 1;
    machine.run();
    println!("Register a for c = 1: {}", machine.regs[0]);
}
