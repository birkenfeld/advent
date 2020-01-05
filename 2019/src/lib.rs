use std::collections::HashMap;
use std::sync::Arc;

/// Reasons for the intcode machine to stop processing
pub enum IO {
    Input,
    Output(i64),
    Halt,
}

/// The Intcode computer.
#[derive(Clone)]
pub struct Machine {
    ip: i64,
    bp: i64,
    mem: Arc<Box<[i64]>>,
    wmem: HashMap<i64, i64>,
    input: Vec<i64>,
}

impl Machine {
    /// Parse stringified intcode.
    pub fn parse(code: &str) -> Arc<Box<[i64]>> {
        Arc::new(code.trim().split(',').map(|v| v.parse().expect("invalid memory")).collect())
    }

    /// Create a new machine with given memory cells and initial input.
    pub fn new(mem: &Arc<Box<[i64]>>) -> Self {
        Self { ip: 0, bp: 0, mem: mem.clone(), wmem: HashMap::new(), input: Vec::new() }
    }

    /// Add some input to the machine.
    pub fn with_input(mut self, new_input: i64) -> Self {
        self.input.push(new_input);
        self
    }

    /// Add multiple inputs, in string form, to the machine.
    pub fn with_input_str(mut self, new_input: &str) -> Self {
        for ch in new_input.chars() {
            self.input.push(ch as i64);
        }
        self
    }

    /// Run the machine with some new input until it produces some output.
    pub fn next_with(&mut self, new_input: i64) -> Option<i64> {
        self.input.push(new_input);
        self.next()
    }

    /// Return a mutable reference to the input source.
    pub fn push_input(&mut self, new_input: i64) {
        self.input.push(new_input);
    }

    /// Return contents of given memory cell.
    #[inline]
    pub fn mem(&self, index: i64) -> i64 {
        self.wmem.get(&index).or_else(|| self.mem.get(index as usize)).copied().unwrap_or(0)
    }

    /// Set contents of given memory cell.
    #[inline]
    pub fn set_mem(&mut self, index: i64, value: i64) {
        // We could check if mem already contains the correct value and avoid
        // populating the wmem hashmap, but it is worse across all benchmarks.
        self.wmem.insert(index, value);
    }

    #[inline]
    fn set_par(&mut self, mode: i64, val: i64) {
        let arg_addr = self.ip - 1;
        let addr = match mode {
            0 => self.mem(arg_addr),
            2 => self.bp + self.mem(arg_addr),
            1 => unreachable!("can't output in immediate mode"),
            _ => unreachable!("invalid memory mode {}", mode)
        };
        self.set_mem(addr, val);
    }

    #[inline]
    fn par(&self, mode: i64, off: i64) -> i64 {
        let arg_addr = self.ip + off;
        self.mem(match mode {
            0 => self.mem(arg_addr),
            1 => arg_addr,
            2 => self.bp + self.mem(arg_addr),
            _ => unreachable!("invalid memory mode {}", mode)
        })
    }

    #[inline]
    fn binop<F: Fn(i64, i64) -> i64>(&mut self, m1: i64, m2: i64, m3: i64, op: F) {
        self.ip += 3;
        let vs = self.par(m1, -3);
        let vt = self.par(m2, -2);
        self.set_par(m3, op(vs, vt));
    }

    #[inline]
    fn jumpop<F: Fn(i64) -> bool>(&mut self, m1: i64, m2: i64, cond: F) {
        if cond(self.par(m1, 0)) {
            self.ip = self.par(m2, 1);
        } else {
            self.ip += 2;
        }
    }

    /// Run until an I/O event or halt.
    pub fn run(&mut self) -> IO {
        loop {
            let opcode = self.mem(self.ip);
            self.ip += 1;
            let m3 = opcode / 10000;
            let m2 = (opcode / 1000) % 10;
            let m1 = (opcode / 100) % 10;
            let op = opcode % 100;
            match op {
                1 => self.binop(m1, m2, m3, |a, b| a + b),
                2 => self.binop(m1, m2, m3, |a, b| a * b),
                5 => self.jumpop(m1, m2, |a| a != 0),
                6 => self.jumpop(m1, m2, |a| a == 0),
                7 => self.binop(m1, m2, m3, |a, b| (a < b) as i64),
                8 => self.binop(m1, m2, m3, |a, b| (a == b) as i64),
                9 => { self.ip += 1; self.bp += self.par(m1, -1); },
                3 => {
                    if self.input.is_empty() {
                        self.ip -= 1;
                        return IO::Input;
                    } else {
                        self.ip += 1;
                        let v = self.input.remove(0);
                        self.set_par(m1, v);
                    }
                }
                4 => {
                    self.ip += 1;
                    return IO::Output(self.par(m1, -1));
                }
                99 => return IO::Halt,
                d => panic!("unknown opcode {}", d)
            }
        }
    }
}

/// To implement Iterator, every call to next() produces one piece
/// of output until the machine halts.  Further input must not be
/// requested.
impl Iterator for Machine {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        match self.run() {
            IO::Input => panic!("machine ran out of input"),
            IO::Output(n) => Some(n),
            IO::Halt => None,
        }
    }
}
