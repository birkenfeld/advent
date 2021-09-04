use std::convert::TryFrom;
use advtools::input::input_string;
use advent19::Machine;
use num_enum::TryFromPrimitive;
use rand::{Rng, thread_rng};

/// Total size of the springscript population we're observing.
const TOTAL: usize = 20000;
/// Number of best scripts to keep from every generation.
const KEEP: usize = 3000;
/// Maximum number of instructions per script.
const MAX_INSTR: usize = 15;

/// Source registers for script instructions.
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(usize)]
enum RegR { T, J, A, B, C, D, E, F, G, H, I }

/// Target registers for script instructions.
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(usize)]
enum RegW { T, J }

/// Operations for script instructions.
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(usize)]
enum Op { NOT, AND, OR }

type Instr = (Op, RegR, RegW);
type Script = Vec<Instr>;

/// Generate a random number from 0 up to n-1.
fn rand_n(n: usize) -> usize {
    thread_rng().gen_range(0..n)
}

/// Generate a random source register
fn random_reg_r(regs: usize) -> RegR {
    RegR::try_from(rand_n(regs)).unwrap()
}

/// Generate a random target register.
fn random_reg_w() -> RegW {
    RegW::try_from(rand_n(2)).unwrap()
}

/// Generate a random operation.
fn random_op() -> Op {
    Op::try_from(rand_n(3)).unwrap()
}

/// Check a springscript sequence against a test case, which is a list of
/// floor tiles that are solid (true) or holes (false).
fn check_case(script: &Script, case: &[bool]) -> bool {
    let mut p = 0;
    while p < case.len() {
        if !case[p] {
            return false;
        }
        let (mut t, mut j) = (false, false);
        for instr in script {
            let arg = *match instr.1 {
                RegR::T => Some(&t),
                RegR::J => Some(&j),
                _       => case.get(p + instr.1 as usize - 1)
            }.unwrap_or(&true);
            let target = match instr.2 {
                RegW::T => &mut t,
                RegW::J => &mut j
            };
            match instr.0 {
                Op::NOT => *target = !arg,
                Op::AND => *target &= arg,
                Op::OR => *target |= arg,
            }
        }
        p += if j { 4 } else { 1 };
    }
    true
}

/// Encapsulates the state we need to solve one part of the puzzle.
struct Generator {
    /// Intcode machine to check scripts against.
    machine: Machine,
    /// List of previously seen test cases from the machine.
    cases: Vec<Vec<bool>>,
    /// Number of source registers available in this part.
    regs: usize,
    /// Start instruction for this part.
    start: &'static str,
}

impl Generator {
    fn new(machine: Machine, regs: usize, start: &'static str) -> Self {
        Self { machine, cases: Vec::new(), regs, start }
    }

    fn random_instr(&self) -> Instr {
        (random_op(), random_reg_r(self.regs), random_reg_w())
    }

    fn random_script(&mut self) -> (usize, Script) {
        self.eval((0..rand_n(MAX_INSTR) + 1).map(|_| self.random_instr()).collect())
    }

    /// Assign a score to a script.
    /// The score is the number of test cases from the intcode machine that
    /// the script passes.
    ///
    /// We cache previously seen test cases from the machine, since they are
    /// much, much cheaper to check like that.  Only if all cached cases are
    /// exhausted, the machine is used to find the next one.
    fn eval(&mut self, script: Script) -> (usize, Script) {
        // First try all our cached test cases.
        for (i, case) in self.cases.iter().enumerate() {
            if !check_case(&script, case) {
                return (i, script);
            }
        }

        // All of the cached cases pass, now get the machine and
        // try with this hopeful solution.
        let mut machine = self.machine.clone();
        for (op, rr, rw) in &script {
            machine = machine.with_input_str(&format!("{:?} {:?} {:?}\n", op, rr, rw));
        }
        machine = machine.with_input_str(self.start);

        while let Some(ch) = machine.next() {
            if ch == b'#' as i64 {
                // We got the display for failure.  Collect the floor for this
                // case and add it to the test cases.
                let mut new_case = vec![true];
                for x in machine.by_ref().take(16) {
                    new_case.push(x == b'#' as i64);
                }
                self.cases.push(new_case);
                return (self.cases.len() - 1, script);
            }
            if ch > 255 {
                // We got a large value, it must be the damage.
                return (ch as usize, script);
            }
        }
        panic!("machine produced unexpected output")
    }

    /// Mutate and score a given script.
    fn mutate_script(&mut self, mut instrs: Script) -> (usize, Script) {
        // We can mutate random existing instructions by exchanging operation,
        // source or target register.
        for instr in &mut instrs {
            if rand_n(8) == 0 {
                if rand_n(3) == 0 {
                    instr.0 = random_op();
                } else if rand_n(2) == 0 {
                    instr.1 = random_reg_r(self.regs);
                } else {
                    instr.2 = random_reg_w();
                }
            }
        }
        // We can add an instruction if the length is not maximal.
        if instrs.len() < MAX_INSTR && rand_n(4) == 0 {
            instrs.insert(rand_n(instrs.len()), self.random_instr());
        // We can remove an instruction otherwise.
        } else if instrs.len() > 1  && rand_n(3) == 0 {
            instrs.remove(rand_n(instrs.len()));
        }
        self.eval(instrs)
    }

    /// Run the main genetic algorithm:
    ///
    /// - create a population of random scripts
    /// - score them according to our metric (number of test cases passed)
    /// - keep some percentage, and add mutations of the best scripts
    /// - repeat until we find a winner
    fn run(&mut self) -> usize {
        let mut pop = Vec::new();
        let mut no_change = 8; // start with a fresh population
        let mut last_best = 0;

        loop {
            // If the best score hasn't changed for a while, start with a
            // completely fresh population.
            if no_change >= 8 {
                pop = (0..TOTAL).map(|_| self.random_script()).collect();
                no_change = 0;
            } else {
                // Keep the best N, add N randoms, and generate mutations of the
                // best N for the rest of the population.
                pop.truncate(KEEP);
                pop.extend((0..KEEP).map(|_| self.random_script()));
                for i in 0..TOTAL - 2*KEEP {
                    pop.push(self.mutate_script(pop[i % KEEP].1.clone()));
                }
            }

            pop.sort_by_key(|v| usize::max_value() - v.0);
            let best = pop.first().unwrap().0;
            if best > 1000 {
                // This is a hull damage value, we're done.
                return best;
            } else if best == last_best {
                no_change += 1;
            } else {
                no_change = 0;
                last_best = best;
            }
        }
    }
}

fn main() {
    let code = Machine::parse(&input_string());

    let mut gen1 = Generator::new(Machine::new(&code), 6, "WALK\n");
    advtools::verify("Hull damage", gen1.run(), 19347868);

    let mut gen2 = Generator::new(Machine::new(&code), 11, "RUN\n");
    advtools::verify("Hull damage with extended range", gen2.run(), 1142479667);
}
