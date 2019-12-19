use std::collections::HashMap;
use std::str::FromStr;
use num::BigInt;
use num::{Integer, ToPrimitive, traits::NumOps};

pub type Mem<Int> = HashMap<usize, Int>;

/// The Intcode computer.
pub struct Machine<Int> {
    ip: usize,
    bp: Int,
    mem: Mem<Int>,
    zero: Int,
    input: Vec<Int>,
}

pub type I32Machine = Machine<i32>;
pub type I64Machine = Machine<i64>;
pub type BigMachine = Machine<BigInt>;

impl<Int: Integer + FromStr + Clone + Default + ToPrimitive + From<u8>> Machine<Int>
where <Int as FromStr>::Err: std::fmt::Debug, for<'a> &'a Int: NumOps<&'a Int, Int>
{
    pub fn parse(code: &str) -> Mem<Int> {
        code.trim().split(',')
                   .enumerate()
                   .map(|(i, v)| (i.into(), v.parse().expect("invalid memory")))
                   .collect()
    }

    /// Create a new machine with given memory cells and initial input.
    pub fn new(mem: &Mem<Int>) -> Self {
        Self { ip: 0, bp: Int::zero(), zero: Int::zero(),
               mem: mem.clone(), input: Vec::new() }
    }

    /// Add some input to the machine.
    pub fn with_input<N, I>(mut self, new_input: I) -> Self
    where I: IntoIterator<Item=N>, N: Into<Int>
    {
        self.input.extend(new_input.into_iter().map(Into::into));
        self
    }

    /// Run the machine with some new input until it produces some output.
    pub fn run<N, I>(&mut self, new_input: I) -> Option<Int>
    where I: IntoIterator<Item=N>, N: Into<Int>
    {
        self.input.extend(new_input.into_iter().map(Into::into));
        self.next()
    }

    /// Return contents of given memory cell.
    pub fn mem(&self, index: usize) -> Int {
        self.mem.get(&index).cloned().unwrap_or_default()
    }

    /// Set contents of given memory cell.
    pub fn set_mem(&mut self, index: usize, value: impl Into<Int>) {
        self.mem.insert(index, value.into());
    }

    fn reg(&mut self, mode: u32) -> &mut Int {
        self.ip += 1;
        if mode == 0 {
            // Address mode
            let addr = Self::to_addr(&self.mem[&(self.ip - 1)]);
            self.mem.entry(addr).or_default()
        } else if mode == 2 {
            // Relative address mode
            let addr = &self.bp + &self.mem[&(self.ip - 1)];
            self.mem.entry(Self::to_addr(&addr)).or_default()
        } else {
            // Immediate mode
            self.mem.entry(self.ip - 1).or_default()
        }
    }

    fn reg_imm(&self, mode: u32, off: usize) -> &Int {
        let arg = self.mem.get(&(self.ip - 1 + off)).unwrap_or(&self.zero);
        if mode == 0 {
            self.mem.get(&Self::to_addr(&arg)).unwrap_or(&self.zero)
        } else if mode == 2 {
            self.mem.get(&Self::to_addr(&(&self.bp + arg))).unwrap_or(&self.zero)
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
            self.ip = Self::to_addr(self.reg_imm(m2, 2));
        } else {
            self.ip += 2;
        }
    }

    #[inline]
    fn to_addr(i: &Int) -> usize {
        i.to_usize().unwrap()
    }
}

/// To implement Iterator, every call to next() produces one piece
/// of output until the machine halts.
impl<Int: Integer + FromStr + Clone + Default + ToPrimitive + From<u8>> Iterator for Machine<Int>
where <Int as FromStr>::Err: std::fmt::Debug, for<'a> &'a Int: NumOps<&'a Int, Int>
{
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
                7 => self.binop(m1, m2, m3, |a, b| ((a < b) as u8).into()),
                8 => self.binop(m1, m2, m3, |a, b| ((a == b) as u8).into()),
                9 => { let rel = self.reg(m1).clone();
                       self.bp = &self.bp + &rel; },
                99 => return None,
                d => panic!("unknown opcode {}", d)
            }
        }
    }
}
