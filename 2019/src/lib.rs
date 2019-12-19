use std::collections::HashMap;
use num::BigInt;
use num::{Zero, ToPrimitive};

pub type Int = BigInt;
pub type Mem = HashMap<usize, Int>;

/// The Intcode computer.
pub struct Machine {
    ip: usize,
    bp: Int,
    mem: Mem,
    zero: Int,
    input: Vec<Int>,
}

impl Machine {
    pub fn parse(code: &str) -> Mem {
        code.trim().split(',')
                   .enumerate()
                   .map(|(i, v)| (i.into(), v.parse().expect("invalid memory")))
                   .collect()
    }

    /// Create a new machine with given memory cells and initial input.
    pub fn new(mem: &Mem) -> Self {
        Self { ip: 0, bp: BigInt::zero(), zero: BigInt::zero(),
               mem: mem.clone(), input: Vec::new() }
    }

    /// Add some input to the machine.
    pub fn with_input<N, I>(mut self, new_input: I) -> Self
    where I: IntoIterator<Item=N>, N: Into<BigInt>
    {
        self.input.extend(new_input.into_iter().map(Into::into));
        self
    }

    /// Run the machine with some new input until it produces some output.
    pub fn run<N, I>(&mut self, new_input: I) -> Option<Int>
    where I: IntoIterator<Item=N>, N: Into<BigInt>
    {
        self.input.extend(new_input.into_iter().map(Into::into));
        self.next()
    }

    /// Return contents of given memory cell.
    pub fn mem(&self, index: usize) -> Int {
        self.mem.get(&index).cloned().unwrap_or_default()
    }

    /// Set contents of given memory cell.
    pub fn set_mem(&mut self, index: usize, value: impl Into<BigInt>) {
        self.mem.insert(index, value.into());
    }

    fn reg(&mut self, mode: u32) -> &mut Int {
        self.ip += 1;
        if mode == 0 {
            // Address mode
            let addr = self.mem[&(self.ip - 1)].to_usize().unwrap();
            self.mem.entry(addr).or_default()
        } else if mode == 2 {
            // Relative address mode
            let addr = &self.bp + &self.mem[&(self.ip - 1)];
            self.mem.entry(addr.to_usize().unwrap()).or_default()
        } else {
            // Immediate mode
            self.mem.entry(self.ip - 1).or_default()
        }
    }

    fn reg_imm(&self, mode: u32, off: usize) -> &Int {
        let arg = self.mem.get(&(self.ip - 1 + off)).unwrap_or(&self.zero);
        if mode == 0 {
            self.mem.get(&arg.to_usize().unwrap()).unwrap_or(&self.zero)
        } else if mode == 2 {
            self.mem.get(&(&self.bp + arg).to_usize().unwrap()).unwrap_or(&self.zero)
        } else {
            arg
        }
    }

    fn binop<F: Fn(&Int, &Int) -> Int>(&mut self, m1: u32, m2: u32, m3: u32, op: F) {
        let vs = self.reg_imm(m1, 1);
        let vt = self.reg_imm(m2, 2);
        let val = op(vs, vt);
        self.ip += 2;
        *self.reg(m3) = val;
    }

    fn jumpop<F: Fn(&Int) -> bool>(&mut self, m1: u32, m2: u32, cond: F) {
        if cond(self.reg_imm(m1, 1)) {
            self.ip = self.reg_imm(m2, 2).to_usize().unwrap();
        } else {
            self.ip += 2;
        }
    }
}

/// To implement Iterator, every call to next() produces one piece
/// of output until the machine halts.
impl Iterator for Machine {
    type Item = Int;

    fn next(&mut self) -> Option<Int> {
        loop {
            let opcode = self.mem.get(&self.ip).expect("no more code?").to_u32().unwrap();
            self.ip += 1;
            let m3 = opcode / 10000;
            let m2 = (opcode / 1000) % 10;
            let m1 = (opcode / 100) % 10;
            let op = opcode % 100;
            match op {
                1 => self.binop(m1, m2, m3, |a, b| a + b),
                2 => self.binop(m1, m2, m3, |a, b| a * b),
                3 => *self.reg(m1) = self.input.remove(0),
                4 => return Some(self.reg(m1).clone()),
                5 => self.jumpop(m1, m2, |a| !a.is_zero()),
                6 => self.jumpop(m1, m2, |a| a.is_zero()),
                7 => self.binop(m1, m2, m3, |a, b| ((a < b) as u32).into()),
                8 => self.binop(m1, m2, m3, |a, b| ((a == b) as u32).into()),
                9 => { let rel = self.reg(m1).clone(); self.bp += rel; },
                99 => return None,
                d => panic!("unknown opcode {}", d)
            }
        }
    }
}
