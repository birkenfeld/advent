pub type Int = i32;

pub struct Machine {
    ip: usize,
    cells: Vec<Int>,
    input: Vec<Int>,
}

impl Machine {
    pub fn new<I>(cells: &[Int], input: I) -> Self
    where I: IntoIterator<Item=i32>
    {
        Self { ip: 0, cells: cells.to_vec(), input: input.into_iter().collect() }
    }

    pub fn run<I>(&mut self, new_input: I) -> Option<Int>
    where I: IntoIterator<Item=i32>
    {
        self.input.extend(new_input.into_iter());
        self.next()
    }

    pub fn mem(&self, index: usize) -> Int {
        self.cells[index]
    }

    fn reg(&mut self, mode: Int) -> &mut Int {
        self.ip += 1;
        if mode == 0 {
            let addr = self.cells[self.ip - 1];
            &mut self.cells[addr as usize]
        } else {
            &mut self.cells[self.ip - 1]
        }
    }

    fn binop<F: Fn(Int, Int) -> Int>(&mut self, m1: Int, m2: Int, op: F) {
        let vs = *self.reg(m1);
        let vt = *self.reg(m2);
        *self.reg(0) = op(vs, vt);
    }

    fn jumpop<F: Fn(Int) -> bool>(&mut self, m1: Int, m2: Int, cond: F) {
        if cond(*self.reg(m1)) {
            self.ip = *self.reg(m2) as usize;
        } else {
            self.ip += 1;
        }
    }
}

impl Iterator for Machine {
    type Item = Int;

    fn next(&mut self) -> Option<Int> {
        loop {
            let opcode = self.cells[self.ip];
            self.ip += 1;
            let m2 = opcode / 1000;
            let m1 = (opcode / 100) % 10;
            let op = opcode % 100;
            match op {
                1 => self.binop(m1, m2, |a, b| a + b),
                2 => self.binop(m1, m2, |a, b| a * b),
                3 => *self.reg(0) = self.input.remove(0),
                4 => return Some(*self.reg(m1)),
                5 => self.jumpop(m1, m2, |a| a != 0),
                6 => self.jumpop(m1, m2, |a| a == 0),
                7 => self.binop(m1, m2, |a, b| (a < b) as Int),
                8 => self.binop(m1, m2, |a, b| (a == b) as Int),
                99 => return None,
                d => panic!("unknown opcode {}", d)
            }
        }
    }
}
