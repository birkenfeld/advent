use std::collections::HashMap;

pub type Mem = HashMap<i64, i64>;

/// The Intcode computer.
#[derive(Clone)]
pub struct Machine {
    ip: i64,
    bp: i64,
    mem: Mem,
    input: Vec<i64>,
}

impl Machine {
    pub fn parse(code: &str) -> Mem {
        code.trim().split(',')
                   .enumerate()
                   .map(|(i, v)| (i as i64, v.parse().expect("invalid memory")))
                   .collect()
    }

    /// Create a new machine with given memory cells and initial input.
    pub fn new(mem: &Mem) -> Self {
        Self { ip: 0, bp: 0, mem: mem.clone(), input: Vec::new() }
    }

    /// Add some input to the machine.
    pub fn with_input<I>(mut self, new_input: I) -> Self
    where I: IntoIterator<Item=i64>
    {
        self.input.extend(new_input.into_iter());
        self
    }

    /// Run the machine with some new input until it produces some output.
    pub fn run<I>(&mut self, new_input: I) -> Option<i64>
    where I: IntoIterator<Item=i64>
    {
        self.input.extend(new_input.into_iter());
        self.next()
    }

    /// Return contents of given memory cell.
    pub fn mem(&self, index: i64) -> i64 {
        self.mem.get(&index).copied().unwrap_or(0)
    }

    /// Set contents of given memory cell.
    pub fn set_mem(&mut self, index: i64, value: i64) {
        self.mem.insert(index, value);
    }

    fn reg(&mut self, mode: i64) -> &mut i64 {
        self.ip += 1;
        if mode == 0 {
            // Address mode
            let addr = self.mem[&(self.ip - 1)];
            self.mem.entry(addr).or_default()
        } else if mode == 2 {
            // Relative address mode
            let addr = &self.bp + &self.mem[&(self.ip - 1)];
            self.mem.entry(addr).or_default()
        } else {
            // Immediate mode
            self.mem.entry(self.ip - 1).or_default()
        }
    }

    fn reg_imm(&self, mode: i64, off: i64) -> i64 {
        let arg = self.mem.get(&(self.ip - 1 + off)).unwrap_or(&0);
        if mode == 0 {
            self.mem.get(arg).copied().unwrap_or(0)
        } else if mode == 2 {
            self.mem.get(&(self.bp + arg)).copied().unwrap_or(0)
        } else {
            *arg
        }
    }

    fn binop<F: Fn(i64, i64) -> i64>(&mut self, m1: i64, m2: i64, m3: i64, op: F) {
        let vs = self.reg_imm(m1, 1);
        let vt = self.reg_imm(m2, 2);
        let val = op(vs, vt);
        self.ip += 2;
        *self.reg(m3) = val;
    }

    fn jumpop<F: Fn(i64) -> bool>(&mut self, m1: i64, m2: i64, cond: F) {
        if cond(self.reg_imm(m1, 1)) {
            self.ip = self.reg_imm(m2, 2);
        } else {
            self.ip += 2;
        }
    }
}

/// To implement Iterator, every call to next() produces one piece
/// of output until the machine halts.
impl Iterator for Machine {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        loop {
            let opcode = self.mem.get(&self.ip).expect("no more code?");
            self.ip += 1;
            let m3 = opcode / 10000;
            let m2 = (opcode / 1000) % 10;
            let m1 = (opcode / 100) % 10;
            let op = opcode % 100;
            match op {
                1 => self.binop(m1, m2, m3, |a, b| a + b),
                2 => self.binop(m1, m2, m3, |a, b| a * b),
                3 => *self.reg(m1) = self.input.remove(0),
                4 => return Some(*self.reg(m1)),
                5 => self.jumpop(m1, m2, |a| a != 0),
                6 => self.jumpop(m1, m2, |a| a == 0),
                7 => self.binop(m1, m2, m3, |a, b| (a < b) as i64),
                8 => self.binop(m1, m2, m3, |a, b| (a == b) as i64),
                9 => { let rel = *self.reg(m1); self.bp += rel; },
                99 => return None,
                d => panic!("unknown opcode {}", d)
            }
        }
    }
}
