use advtools::prelude::Itertools;
use advtools::input::input_string;

type Int = i32;

const LANDING: Int = 19690720;

struct Machine {
    ip: usize,
    cells: Vec<Int>,
}

impl Machine {
    fn new(cells: &[Int], noun: Int, verb: Int) -> Self {
        let mut cells = cells.to_vec();
        cells[1] = noun;
        cells[2] = verb;
        Self { ip: 0, cells }
    }

    fn run(&mut self) -> Int {
        loop {
            match self.cells[self.ip] {
                99 => return self.cells[0],
                1 => self.binop(|a, b| a + b),
                2 => self.binop(|a, b| a * b),
                d => panic!("unknown opcode {}", d)
            }
        }
    }

    fn binop<F: Fn(Int, Int) -> Int>(&mut self, op: F) {
        let rd = self.cells[self.ip + 3] as usize;
        self.cells[rd] = op(self.cells[self.cells[self.ip + 1] as usize],
                            self.cells[self.cells[self.ip + 2] as usize]);
        self.ip += 4;
    }
}

fn main() {
    let cells = input_string().trim().split(',').map(|v| v.parse().unwrap()).collect_vec();

    // Part 1: just run with a given noun/verb combination.
    advtools::print("First round", Machine::new(&cells, 12, 2).run());

    // Part 2: try different nouns/verbs to get the desired landing date.
    for noun in 0..100 {
        for verb in 0..100 {
            if Machine::new(&cells, noun, verb).run() == LANDING {
                advtools::print("Second round", 100*noun + verb);
                break;
            }
        }
    }
}
