use std::collections::HashSet;
use advtools::rayon::{self, prelude::*};

const MAX: usize = 7;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
struct State([u8; 2*MAX+2]);

impl State {
    fn new(mut gens: Vec<u8>, mut chips: Vec<u8>) -> Self {
        assert!(gens.len() <= MAX);
        assert_eq!(gens.len(), chips.len());
        let mut state = State::default();
        state.set_n(gens.len());
        gens.append(&mut chips);
        for (i, floor) in gens.into_iter().enumerate() {
            state.set_thing(i, floor);
        }
        state
    }
    fn n(&self) -> usize {
        self.0[0] as usize
    }
    fn set_n(&mut self, n: usize) {
        self.0[0] = n as u8;
    }
    fn floor(&self) -> u8 {
        self.0[1]
    }
    fn set_floor(&mut self, f: u8) {
        self.0[1] = f;
    }
    fn thing(&self, i: usize) -> u8 {
        self.0[2+i]
    }
    fn set_thing(&mut self, i: usize, f: u8) {
        self.0[2+i] = f;
    }
    fn swap_things(&mut self, i: usize, j: usize) {
        self.0.swap(2+i, 2+j);
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

fn next_states(states: Vec<State>, seen: &HashSet<State>) -> Vec<State> {
    states
        .into_par_iter()
        .map(|state| {
            let mut res = Vec::with_capacity(200);
            // determine and maybe queue all new states
            for new_floor in 0..4 {
                let mut try_state = |mut new_state: State, j1, j2| {
                    new_state.set_floor(new_floor);
                    new_state.set_thing(j1, new_floor);
                    new_state.set_thing(j2, new_floor);
                    if new_state.canonicalize_and_check() && !seen.contains(&new_state) {
                        res.push(new_state);
                    }
                };
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
                        try_state(state, j1, j1);
                        // two-thing moves
                        for j2 in 0..j1 {
                            if state.thing(j2) == state.floor() {
                                try_state(state, j1, j2);
                            }
                        }
                    }
                }
            }
            res
        })
        .reduce(|| Vec::with_capacity(2000), |mut v, mut x| { v.append(&mut x); v })
}

fn find_steps(initial: State) -> Option<usize> {
    let mut seen = HashSet::with_capacity(1_000_000);
    let mut states = vec![initial];
    let mut generation = 0;

    loop {
        let new_states = next_states(states, &seen);
        generation += 1;
        states = Vec::with_capacity(new_states.len());
        for state in new_states {
            // need to check here again to weed out duplicates from the parallel
            // determination of new states (since we can't insert in next_level)
            if !seen.insert(state) {
                continue;
            }
            if state.is_done() {
                return Some(generation);
            }
            states.push(state);
        }
        if states.is_empty() {
            return None;
        }
    }
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    let state1 = State::new(vec![0, 0, 0, 0, 0], vec![1, 1, 0, 0, 0]);
    advtools::verify("Min. # steps (5 chips)", find_steps(state1).unwrap(), 47);
    let state2 = State::new(vec![0, 0, 0, 0, 0, 0, 0], vec![1, 1, 0, 0, 0, 0, 0]);
    advtools::verify("Min. # steps (7 chips)", find_steps(state2).unwrap(), 71);
}
