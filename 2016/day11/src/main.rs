extern crate arrayvec;
extern crate fnv;

use arrayvec::{Array, ArrayVec};
use std::collections::VecDeque;
use std::hash::Hash;

const F: u8 = 4;

trait ByteArray: Array<Item=u8> + Clone + Hash + Eq {}
impl<T: Array<Item=u8> + Clone + Hash + Eq> ByteArray for T {}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State<A: ByteArray> {
    floor:  u8,
    things: ArrayVec<A>,
}

impl<A: ByteArray> State<A> {
    fn is_done(&self) -> bool {
        self.floor == F-1 && self.things.iter().all(|&floor| floor == F-1)
    }
    fn canonicalize_and_check(&mut self, n: usize) -> bool {
        // if not ok, no need to do the sorting
        if !self.things[n..].iter().enumerate().all(
            |(i, &floor)| self.things[i] == floor ||
                !self.things[..n].iter().any(|&v| v == floor)) {
            return false;
        }
        // unsophisticated O(n^2) insertionsort, does the job
        for i in 1..n {
            let mut j = i;
            while j > 0 && self.things[j-1] > self.things[j] {
                self.things.swap(j, j-1);
                self.things.swap(j+n, j-1+n);
                j -= 1;
            }
        }
        true
    }
}

fn find_steps<A: ByteArray>(n: usize, initial: State<A>) -> Option<usize> {
    let mut queue = VecDeque::with_capacity(20000);
    let mut queued = fnv::FnvHashMap::with_capacity_and_hasher(1000000, Default::default());
    queue.push_back((initial, 0));
    while let Some((state, i)) = queue.pop_front() {
        // check for found solution
        if state.is_done() {
            return Some(i);
        }
        // determine and maybe queue all new states
        let mut try_state = |mut new_state: State<A>, new_floor, j1, j2| {
            new_state.floor = new_floor;
            new_state.things[j1] = new_floor;
            new_state.things[j2] = new_floor;
            if new_state.canonicalize_and_check(n) && !queued.contains_key(&new_state) {
                queued.insert(new_state.clone(), i+1);
                queue.push_back((new_state, i+1));
            }
        };
        for new_floor in 0..4 {
            // only move to adjacent floors
            if !(new_floor + 1 == state.floor || new_floor == state.floor + 1) {
                continue;
            }
            // don't move down if all floors below are empty
            if new_floor < state.floor && state.things.iter().all(|&v| v >= state.floor) {
                continue;
            }
            for j1 in 0..2*n {
                if state.things[j1] == state.floor {
                    try_state(state.clone(), new_floor, j1, j1);

                    for j2 in 0..j1 {
                        if state.things[j2] == state.floor {
                            try_state(state.clone(), new_floor, j1, j2);
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let state1 = State { floor:  0, things: [0, 0, 0, 0, 0, 1, 1, 0, 0, 0].into() };
    println!("Min. # steps (5 chips): {:?}", find_steps(5, state1).unwrap());
    let state2 = State { floor:  0, things: [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0].into() };
    println!("Min. # steps (7 chips): {:?}", find_steps(7, state2).unwrap());
}
