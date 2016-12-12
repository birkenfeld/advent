use std::collections::{VecDeque, HashSet};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State(u64);

impl State {
    fn new(gens: &[u64], chips: &[u64]) -> Self {
        assert_eq!(gens.len(), chips.len());
        let n = gens.len();
        let mut num = (n as u64) << 58;
        for (i, &gen) in gens.iter().enumerate() {
            num |= gen << (i << 2);
        }
        for (i, &chip) in chips.iter().enumerate() {
            num |= chip << ((i + n) << 2);
        }
        State(num)
    }
    fn n(&self) -> usize {
        (self.0 >> 58) as usize
    }
    fn thing(&self, i: usize) -> u64 {
        (self.0 >> (i << 2)) & 0x3
    }
    fn set_thing(&mut self, i: usize, f: u64) {
        self.0 = (self.0 & !(3 << (i << 2))) | (f << (i << 2));
    }
    fn floor(&self) -> u64 {
        self.thing(14)
    }
    fn set_floor(&mut self, f: u64) {
        self.set_thing(14, f);
    }
    fn swap_things(&mut self, i: usize, j: usize) {
        let a = self.thing(i);
        let b = self.thing(j);
        self.set_thing(i, b);
        self.set_thing(j, a);
    }
    fn all_above(&self) -> bool {
        (0..self.n()*2).all(|i| self.thing(i) >= self.floor())
    }
    fn is_done(&self) -> bool {
        self.floor() == 3 && (0..2*self.n()).all(|i| self.thing(i) == 3)
    }
    fn canonicalize_and_check(&mut self) -> bool {
        let n = self.n();
        // check if state is bad, no need to do the sorting if yes
        if !(0..n).all(|i| self.thing(i+n) == self.thing(i) ||
                       !(0..n).any(|j| self.thing(j) == self.thing(i+n))) {
            return false;
        }
        // sort state - since all gens/chips are equal they are exchangeable
        // unsophisticated O(n^2) insertionsort, does the job
        for i in 1..n {
            let mut j = i;
            while j > 0 && self.thing(j-1) > self.thing(j) {
                self.swap_things(j, j-1);
                self.swap_things(j+n, j-1+n);
                j -= 1;
            }
        }
        true
    }
}

fn find_steps(initial: State) -> Option<usize> {
    let mut queue = VecDeque::with_capacity(20000);
    let mut queued = HashSet::with_capacity(1000000);
    queue.push_back((initial, 0));
    while let Some((state, i)) = queue.pop_front() {
        // check for found solution
        if state.is_done() {
            return Some(i);
        }
        // determine and maybe queue all new states
        let mut try_state = |mut new_state: State, new_floor, j1, j2| {
            new_state.set_floor(new_floor);
            new_state.set_thing(j1, new_floor);
            new_state.set_thing(j2, new_floor);
            if new_state.canonicalize_and_check() && !queued.contains(&new_state) {
                queued.insert(new_state);
                queue.push_back((new_state, i+1));
            }
        };
        for new_floor in 0..4 {
            // only move to adjacent floors
            if !(new_floor + 1 == state.floor() || new_floor == state.floor() + 1) {
                continue;
            }
            // don't move down if all floors below are empty
            if new_floor < state.floor() && state.all_above() {
                continue;
            }
            for j1 in 0..2*state.n() {
                if state.thing(j1) == state.floor() {
                    // one-thing moves
                    try_state(state, new_floor, j1, j1);
                    // two-thing moves
                    for j2 in 0..j1 {
                        if state.thing(j2) == state.floor() {
                            try_state(state, new_floor, j1, j2);
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let state1 = State::new(&[0, 0, 0, 0, 0], &[1, 1, 0, 0, 0]);
    println!("Min. # steps (5 chips): {:?}", find_steps(state1).unwrap());
    let state2 = State::new(&[0, 0, 0, 0, 0, 0, 0], &[1, 1, 0, 0, 0, 0, 0]);
    println!("Min. # steps (7 chips): {:?}", find_steps(state2).unwrap());
}
