use advtools::prelude::Itertools;
use advtools::input;
use advtools::grid::{Grid, Pos, Dir::*};

/// Minimum grid size we need to not run out.
const SIZE: usize = 401;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State { Clean, Weakened, Infected, Flagged }

/// Run a number of iterations.
///
/// To accomodate the differences between the two parts, there is a closure
/// which determines how states change.
fn run<F>(n: u32, mut grid: Grid<State>, modify: F) -> u32 where F: Fn(State) -> State {
    let mut pos = Pos(SIZE/2, SIZE/2);  // start in the center
    let mut dir = U;
    let mut infections = 0;
    for _ in 0..n {
        let state = grid[pos];
        dir = match state {
            State::Clean    => dir.left(),
            State::Weakened => dir,
            State::Infected => dir.right(),
            State::Flagged  => dir.flip(),
        };
        let new_state = modify(state);
        if new_state == State::Infected {
            infections += 1;
        }
        grid[pos] = new_state;
        pos.step(dir);
    }
    infections
}

fn main() {
    let mut grid = vec![vec![State::Clean; SIZE]; SIZE];
    let input = input::lines().collect_vec();
    let start_offset = input.len() / 2;
    let offset = SIZE/2 - start_offset;
    for (y, line) in input.into_iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                grid[y + offset][x + offset] = State::Infected;
            }
        }
    }
    let grid = Grid::new(grid);

    // Part 1: Run 10k iterations, only Clean <-> Infected.
    let part1 = run(10_000, grid.clone(), |state| match state {
        State::Infected => State::Clean,
        State::Clean => State::Infected,
        _ => unreachable!()
    });
    advtools::verify("Infections part 1", part1, 5369);

    // Part 2: Run 10M iterations, with full four states.
    let part2 = run(10_000_000, grid, |state| match state {
        State::Clean    => State::Weakened,
        State::Weakened => State::Infected,
        State::Infected => State::Flagged,
        State::Flagged  => State::Clean,
    });
    advtools::verify("Infections part 2", part2, 2510774);
}
