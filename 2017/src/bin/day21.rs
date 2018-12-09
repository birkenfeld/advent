use advtools::prelude::{HashMap, Itertools};
use advtools::input::iter_input;
use advtools::rayon::prelude::*;

type S2 = (u8,u8,u8,u8);
type S3 = (u8,u8,u8,u8,u8,u8,u8,u8,u8);

/// We represent the state of the image using rows/cols of squares.
/// Therefore there are three types of states that are run through cyclically,
/// and depend on the requirements of the next replacement:
///
/// One: odd total size -> represented as squares of size 3 that will be
///      replaced by size 4 (split into 2x2)
/// Two: even total size -> squares of size 2 of which a 2x2 group will be
///      replaced by size 3s and then reordered in 3x3 size 2s
/// Three: even total size -> squares of size 2 which will be replaced by
///      squares of size 3
enum State {
    One(Vec<Vec<S3>>),
    Two(Vec<Vec<S2>>),
    Three(Vec<Vec<S2>>),
}

impl State {
    /// Sum up all set pixels in a state.
    fn sum(&self) -> u32 {
        match self {
            State::One(v) =>
                v.iter().map(|w| w.iter().map(
                    |i| (i.0 + i.1 + i.2 + i.3 + i.4 + i.5 + i.6 + i.7 + i.8) as u32
                ).sum::<u32>()).sum(),
            State::Two(v) | State::Three(v) =>
                v.iter().map(|w| w.iter().map(
                    |i| (i.0 + i.1 + i.2 + i.3) as u32
                ).sum::<u32>()).sum(),
        }
    }
}

fn pixel(ch: char) -> u8 {
    match ch {
        '.' => 0,
        '#' => 1,
        _ => unreachable!()
    }
}

/// Collect a vector into a tuple.
fn square2x2(v: Vec<u8>) -> S2 {
    (v[0], v[1], v[2], v[3])
}

/// Collect a vector into a tuple.
fn square3x3(v: Vec<u8>) -> S3 {
    (v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8])
}

/// Flip a 2x2 pattern.
fn flip2x2(v: S2) -> S2 {
    (v.1, v.0,
     v.3, v.2)
}

/// Rotate a 2x2 pattern by 90deg.
fn rotate2x2(v: S2) -> S2 {
    (v.2, v.0,
     v.3, v.1)
}

/// Flip a 3x3 pattern.
fn flip3x3(v: S3) -> S3 {
    (v.2, v.1, v.0,
     v.5, v.4, v.3,
     v.8, v.7, v.6)
}

/// Rotate a 3x3 pattern by 90deg.
fn rotate3x3(v: S3) -> S3 {
    (v.6, v.3, v.0,
     v.7, v.4, v.1,
     v.8, v.5, v.2)
}

/// Advance the state by one iteration.
fn advance(state: State, repls2x2: &HashMap<S2, S3>, repls3x3: &HashMap<S3, (S2,S2,S2,S2)>) -> State {
    match state {
        State::One(state) => {
            State::Two(state.into_par_iter().flat_map(|row| {
                let mut new1 = Vec::new();
                let mut new2 = Vec::new();
                for square in row {
                    // The 4x4 replacement for 3x3 squares is already prepared as 4 2x2 squares.
                    let (repl1, repl2, repl3, repl4) = repls3x3[&square];
                    new1.push(repl1);
                    new1.push(repl2);
                    new2.push(repl3);
                    new2.push(repl4);
                }
                vec![new1, new2]
            }).collect())
        }
        State::Two(state) => {
            State::Three(state.par_chunks(2).flat_map(|rows| {
                let mut new1 = Vec::new();
                let mut new2 = Vec::new();
                let mut new3 = Vec::new();
                // Iterate over 2x2 size-2 squares at at the same time.
                for ((sq1, sq2), (sq3, sq4)) in rows[0].iter().tuples().zip(rows[1].iter().tuples()) {
                    // Get the 2x2 size-3 squares and make them into 3x3 size-2 squares.
                    let repl1 = &repls2x2[&sq1];
                    let repl2 = &repls2x2[&sq2];
                    let repl3 = &repls2x2[&sq3];
                    let repl4 = &repls2x2[&sq4];
                    new1.push((repl1.0, repl1.1, repl1.3, repl1.4));
                    new1.push((repl1.2, repl2.0, repl1.5, repl2.3));
                    new1.push((repl2.1, repl2.2, repl2.4, repl2.5));
                    new2.push((repl1.6, repl1.7, repl3.0, repl3.1));
                    new2.push((repl1.8, repl2.6, repl3.2, repl4.0));
                    new2.push((repl2.7, repl2.8, repl4.1, repl4.2));
                    new3.push((repl3.3, repl3.4, repl3.6, repl3.7));
                    new3.push((repl3.5, repl4.3, repl3.8, repl4.6));
                    new3.push((repl4.4, repl4.5, repl4.7, repl4.8));
                }
                vec![new1, new2, new3]
            }).collect())
        }
        State::Three(state) => {
            State::One(state.into_par_iter().map(|row| {
                row.into_iter().map(|square| repls2x2[&square]).collect()
            }).collect())
        }
    }
}

/// Add a pattern to the replacement mapping.
///
/// There are at most 8 different ways to rotate/flip the pattern and end up
/// with distinct new patterns: one set of pi/2 rotations, and another, starting
/// with a flipped version.
fn add_to_map<P, Q>(mut pat: P, repl: Q, flip: fn(P) -> P, rot: fn(P) -> P, map: &mut HashMap<P, Q>)
    where P: std::hash::Hash + Eq + Copy, Q: Copy
{
    for _ in 0..4 {
        map.insert(pat, repl);
        map.insert(flip(pat), repl);
        pat = rot(pat);
    }
}

fn main() {
    let mut repls2x2 = HashMap::new();
    let mut repls3x3 = HashMap::new();
    for line in iter_input::<Vec<String>>() {
        let rpat = line[0].chars().filter(|&c| c != '/').map(pixel).collect_vec();
        let repl = line[2].chars().filter(|&c| c != '/').map(pixel).collect_vec();
        if line[0].len() == 5 {
            add_to_map(square2x2(rpat), square3x3(repl), flip2x2, rotate2x2, &mut repls2x2);
        } else {
            // Process the size-4 square into the 2x2 grid of size-2 squares
            // we need later anyway.
            let repls = (
                (repl[0],  repl[1],  repl[4],  repl[5]),
                (repl[2],  repl[3],  repl[6],  repl[7]),
                (repl[8],  repl[9],  repl[12], repl[13]),
                (repl[10], repl[11], repl[14], repl[15]),
            );
            add_to_map(square3x3(rpat), repls, flip3x3, rotate3x3, &mut repls3x3);
        }
    }

    let mut state = State::One(vec![vec![(0, 1, 0,
                                          0, 0, 1,
                                          1, 1, 1)]]);
    // Part 1: five iterations.
    for _ in 0..5 {
        state = advance(state, &repls2x2, &repls3x3);
    }
    advtools::print("Lights on after 5", state.sum());

    // Part 2: 18 iterations.
    for _ in 5..18 {
        state = advance(state, &repls2x2, &repls3x3);
    }
    advtools::print("Lights on after 18", state.sum());
}
