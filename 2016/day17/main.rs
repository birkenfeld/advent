extern crate crypto;
extern crate rayon;

use std::fmt;
use crypto::md5::Md5;
use crypto::digest::Digest;
use rayon::prelude::*;

const INPUT: &'static [u8] = b"edjrjqaa";

#[derive(Clone, Copy)]
enum Dir { U, D, L, R }
use Dir::*;

impl Dir {
    fn from_int(i: u64) -> Self {
        match i {
            0 => U,
            1 => D,
            2 => L,
            3 => R,
            _ => unreachable!()
        }
    }
    fn as_bytes(&self) -> &'static [u8] {
        match *self {
            U => b"U",
            D => b"D",
            L => b"L",
            R => b"R",
        }
    }
}

#[derive(Clone, Default)]
struct State(usize, Vec<u64>);

impl State {
    fn len(&self) -> usize {
        self.0 >> 4
    }
    fn pos(&self) -> usize {
        self.0 & 0xf
    }
    fn dir(&self, idx: usize) -> Dir {
        let (ai, idx) = (idx / 32, idx % 32);
        Dir::from_int((self.1[ai] >> (idx * 2)) & 0x3)
    }
    fn move_dir(&self, dir: Dir) -> State {
        let len = self.len();
        let (ai, idx) = (len / 32, len % 32);
        let mut new = self.clone();
        if idx == 0 {
            new.1.push(dir as u64);
        } else {
            new.1[ai] |= (dir as u64) << (idx * 2);
        }
        new.0 = ((len + 1) << 4) | match dir {
            U => self.pos() - 4,
            D => self.pos() + 4,
            L => self.pos() - 1,
            R => self.pos() + 1,
        };
        new
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len() {
            write!(f, "{}", String::from_utf8_lossy(self.dir(i).as_bytes()))?;
        }
        write!(f, "({})", self.pos())
    }
}

fn eval_hash(mut hash: Md5) -> [bool; 4] {
    let mut buf = [0u8; 16];
    let mut dirs = [false; 4];
    hash.result(&mut buf);
    dirs[0] = (buf[0] >> 4) >= 0xb;
    dirs[1] = (buf[0] & 0xf) >= 0xb;
    dirs[2] = (buf[1] >> 4) >= 0xb;
    dirs[3] = (buf[1] & 0xf) >= 0xb;
    dirs
}

fn next_states(states: Vec<State>) -> Vec<State> {
    states
        .into_par_iter()
        .flat_map(|state| {
            let mut res = Vec::with_capacity(4);
            let mut hash = Md5::new();
            hash.input(INPUT);
            for i in 0..state.len() {
                hash.input(state.dir(i).as_bytes());
            }
            let dirs = eval_hash(hash);
            for (dir, ok) in [U, D, L, R].iter().cloned().zip(&dirs) {
                match (*ok, state.pos(), dir) {
                    (false, _, _) => (),
                    (_, 0, U)  | (_, 1, U)  | (_, 2, U)  | (_, 3, U) |
                    (_, 14, D) | (_, 13, D) | (_, 12, D) |
                    (_, 0, L)  | (_, 4, L)  | (_, 8, L)  | (_, 12, L) |
                    (_, 3, R)  | (_, 7, R)  | (_, 11, R) => (),
                    (_, 15, _) => unreachable!(),

                    _ => res.push(state.move_dir(dir))
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
    rayon::initialize(rayon::Configuration::new().num_threads(3)).unwrap();
    let state = State::default();
    let (final_state, max_path) = find_steps(state);
    println!("Shortest path to goal: {:?}", final_state);
    println!("Max path length: {}", max_path);
}
