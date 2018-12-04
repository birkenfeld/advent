extern crate advtools;
use advtools::prelude::Itertools;
use advtools::input::iter_input;

/// Minimum grid size we need to not run out.
const SIZE: usize = 401;

enum Dir { U, D, L, R }
use self::Dir::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State { Clean, Weakened, Infected, Flagged }

/// Run a number of iterations.
///
/// To accomodate the differences between the two parts, there is a closure
/// which determines how states change.
fn run<F>(n: u32, mut grid: Vec<Vec<State>>, modify: F) -> u32 where F: Fn(State) -> State {
    let (mut x, mut y) = (SIZE/2, SIZE/2);  // start in the center
    let mut dir = U;
    let mut infections = 0;
    for _ in 0..n {
        let state = grid[y][x];
        dir = match state {
            State::Clean    => match dir { U => L, L => D, D => R, R => U },
            State::Weakened => dir,
            State::Infected => match dir { U => R, R => D, D => L, L => U },
            State::Flagged  => match dir { U => D, D => U, R => L, L => R },
        };
        let new_state = modify(state);
        if new_state == State::Infected {
            infections += 1;
        }
        grid[y][x] = new_state;
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
    let mut grid = vec![vec![State::Clean; SIZE]; SIZE];
    let input = iter_input::<String>().collect_vec();
    let start_offset = input.len() / 2;
    let offset = SIZE/2 - start_offset;
    for (y, line) in input.into_iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                grid[y + offset][x + offset] = State::Infected;
            }
        }
    }

    // Part 1: Run 10k iterations, only Clean <-> Infected.
    let part1 = run(10_000, grid.clone(), |state| match state {
        State::Infected => State::Clean,
        State::Clean => State::Infected,
        _ => unreachable!()
    });
    println!("Infections part 1: {}", part1);

    // Part 2: Run 10M iterations, with full four states.
    let part2 = run(10_000_000, grid, |state| match state {
        State::Clean    => State::Weakened,
        State::Weakened => State::Infected,
        State::Infected => State::Flagged,
        State::Flagged  => State::Clean,
    });
    println!("Infections part 2: {}", part2);
}
