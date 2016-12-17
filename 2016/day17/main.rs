extern crate arrayvec;
extern crate crypto;
extern crate rayon;

use std::fmt;
use arrayvec::ArrayVec;
use crypto::md5::Md5;
use crypto::digest::Digest;
use rayon::prelude::*;

const INPUT: &'static [u8] = b"edjrjqaa";

#[derive(Clone)]
struct State(usize, ArrayVec<[u64; 32]>);

impl State {
    fn len(&self) -> usize {
        self.0 >> 4
    }
    fn pos(&self) -> usize {
        self.0 & 0xf
    }
    fn dir(&self, mut idx: usize) -> u8 {
        let mut ai = 0;
        while idx >= 16 {
            ai += 1;
            idx -= 16;
        }
        (self.1[ai] >> (idx * 4)) as u8 & 0x3
    }
    fn move_dir(&self, dir: u64) -> State {
        let len = self.len();
        let mut idx = len;
        let mut ai = 0;
        let mut new = self.clone();
        while idx >= 16 {
            ai += 1;
            idx -= 16;
        }
        if idx == 0 {
            new.1.push(dir);
        } else {
            new.1[ai] |= dir << (idx * 4);
        }
        new.0 = ((len + 1) << 4) | match dir {
            0 => self.pos() - 4,
            1 => self.pos() + 4,
            2 => self.pos() - 1,
            3 => self.pos() + 1,
            _ => unreachable!()
        };
        new
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len() {
            write!(f, "{}", match self.dir(i) {
                0 => 'U',
                1 => 'D',
                2 => 'L',
                3 => 'R',
                _ => unreachable!()
            })?;
        }
        write!(f, "({})", self.pos())
    }
}

const HEXCHARS: &'static [u8] = b"0123456789abcdef";

fn hash_to_hex(hash: &mut Md5, sbuf: &mut [u8; 32]) {
    let mut buf = [0u8; 16];
    hash.result(&mut buf);
    for (i, &byte) in buf.iter().enumerate() {
        sbuf[2*i] = HEXCHARS[(byte >> 4) as usize];
        sbuf[2*i+1] = HEXCHARS[(byte & 0xf) as usize];
    }
}

fn next_states(states: Vec<State>) -> Vec<State> {
    states
        .into_par_iter()
        .flat_map(|state| {
            let mut res = Vec::with_capacity(4);
            let mut hash = Md5::new();
            let mut sbuf = [0u8; 32];
            hash.input(INPUT);
            for i in 0..state.len() {
                hash.input(match state.dir(i) {
                    0 => b"U",
                    1 => b"D",
                    2 => b"L",
                    3 => b"R",
                    _ => unreachable!()
                });
            }
            hash_to_hex(&mut hash, &mut sbuf);
            for dir in 0..4 {
                match (state.pos(), dir) {
                    (15, _) => unreachable!(),
                    (0, 0) | (1, 0) | (2, 0) | (3, 0) |
                    (14, 1) | (13, 1) | (12, 1) |
                    (0, 2) | (4, 2) | (8, 2) | (12, 2) |
                    (3, 3) | (7, 3) | (11, 3) => (),
                    _ => if sbuf[dir] >= b'b' {  // open!
                        let new_state = state.move_dir(dir as u64);
                        res.push(new_state);
                    }
                }
            }
            res
        })
        .collect()
}

fn find_steps(initial: State) -> (State, usize) {
    let mut states = vec![initial];
    let mut max_path = 0;
    let mut shortest = None;

    loop {
        let new_states = next_states(states);
        states = Vec::with_capacity(new_states.len());
        for state in new_states {
            if state.pos() == 0xf {
                if shortest.is_none() {
                    shortest = Some(state.clone());
                }
                max_path = std::cmp::max(max_path, state.len());
            } else {
                states.push(state);
            }
        }
        if states.is_empty() {
            return (shortest.unwrap(), max_path);
        }
    }
}

fn main() {
    rayon::initialize(rayon::Configuration::new().set_num_threads(4)).unwrap();
    let state = State(0, ArrayVec::new());
    let (final_state, max_path) = find_steps(state);
    println!("Shortest path to goal: {:?}", final_state);
    println!("Max path length: {}", max_path);
}
