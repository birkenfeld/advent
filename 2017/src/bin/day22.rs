extern crate advtools;
extern crate fnv;

use advtools::prelude::*;
use fnv::FnvHashMap;

enum Dir { U, D, L, R }
use self::Dir::*;

/// Represents a non-clean state.  We represent "clean" as "not in the map",
/// which essentially makes `Option<State>` the full state of a machine.
#[derive(Clone, Copy, PartialEq, Eq)]
enum State { Weakened, Infected, Flagged }

/// Run a number of iterations.
///
/// To accomodate the differences between the two parts, there is an
/// `init_infect_state` which determines how clean machines change, and a
/// closure which determines how non-clean machines change.
fn run<F>(n: u32, mut map: FnvHashMap<(i32, i32), State>, init_infect_state: State, modify: F) -> u32
    where F: Fn(State) -> Option<State>
{
    let (mut x, mut y) = (0, 0);
    let mut dir = U;
    let mut infections = 0;
    for _ in 0..n {
        match map.entry((x, y)) {
            // Machine was not clean
            Entry::Occupied(mut e) => {
                dir = match *e.get() {
                    State::Weakened => dir,
                    State::Infected => match dir { U => R, R => D, D => L, L => U },
                    State::Flagged  => match dir { U => D, D => U, R => L, L => R },
                };
                let new_state = modify(*e.get());
                if let Some(new_state) = new_state {
                    if new_state == State::Infected {
                        infections += 1;
                    }
                    e.insert(new_state);
                } else {
                    e.remove();
                }
            }
            // Machine was clean
            Entry::Vacant(mut e) => {
                dir = match dir { U => L, L => D, D => R, R => U };
                e.insert(init_infect_state);
                if init_infect_state == State::Infected {
                    infections += 1;
                }
            }
        };
        match dir {
            U => y -= 1,
            D => y += 1,
            L => x -= 1,
            R => x += 1,
        }
    }
    infections
}

fn main() {
    let mut infected = FnvHashMap::default();
    let input = iter_input::<String>().collect_vec();
    let d = input.len() as i32 / 2;
    for (y, line) in input.into_iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                infected.insert(((x as i32) - d, (y as i32) - d), State::Infected);
            }
        }
    }

    // Part 1: Run 10k iterations, only Clean <-> Infected.
    let part1 = run(10_000, infected.clone(), State::Infected, |_| None);
    println!("Infections part 1: {}", part1);

    // Part 2: Run 10M iterations, with full four states.
    let part2 = run(10_000_000, infected, State::Weakened, |state| match state {
        State::Weakened => Some(State::Infected),
        State::Infected => Some(State::Flagged),
        State::Flagged  => None,
    });
    println!("Infections part 2: {}", part2);
}
