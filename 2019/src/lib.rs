pub type Int = i32;

pub struct Machine {
    ip: usize,
    cells: Vec<Int>,
}

impl Machine {
    pub fn new(cells: &[Int], noun_verb: Option<(Int, Int)>) -> Self {
        let mut cells = cells.to_vec();
        if let Some((noun, verb)) = noun_verb {
            cells[1] = noun;
            cells[2] = verb;
        }
        Self { ip: 0, cells }
    }

    pub fn run<I>(&mut self, input: I) -> (Int, Vec<Int>)
        where I: IntoIterator<Item=i32>
    {
        let mut inp = input.into_iter();
        let mut out = Vec::new();
        loop {
            let opcode = self.cells[self.ip];
            self.ip += 1;
            let m2 = opcode / 1000;
            let m1 = (opcode / 100) % 10;
            let op = opcode % 100;
            match op {
                99 => return (self.cells[0], out),
                1 => self.binop(m1, m2, |a, b| a + b),
                2 => self.binop(m1, m2, |a, b| a * b),
                3 => *self.reg(0) = inp.next().unwrap(),
                4 => out.push(*self.reg(m1)),
                5 => self.jumpop(m1, m2, |a| a != 0),
                6 => self.jumpop(m1, m2, |a| a == 0),
                7 => self.binop(m1, m2, |a, b| (a < b) as Int),
                8 => self.binop(m1, m2, |a, b| (a == b) as Int),
                d => panic!("unknown opcode {}", d)
            }
        }
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
