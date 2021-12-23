use advtools::prelude::{Itertools, HashMap};
use advtools::input;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct State<const N: usize> {
    rooms: [[u8; N]; 4],
    hall:  [u8; 7],
}

// Calculate energy cost between a `slot` (in the given `room`) and a `hall` place.
fn energy_cost(kind: u8, room: usize, slot: usize, hall: usize) -> u32 {
    (slot as u32 + match (room, hall) {
        (0, 0) => 3,
        (0, 1) => 2,
        (0, 2) => 2,
        (0, 3) => 4,
        (0, 4) => 6,
        (0, 5) => 8,
        (0, 6) => 9,

        (1, 0) => 5,
        (1, 1) => 4,
        (1, 2) => 2,
        (1, 3) => 2,
        (1, 4) => 4,
        (1, 5) => 6,
        (1, 6) => 7,

        (2, 0) => 7,
        (2, 1) => 6,
        (2, 2) => 4,
        (2, 3) => 2,
        (2, 4) => 2,
        (2, 5) => 4,
        (2, 6) => 5,

        (3, 0) => 9,
        (3, 1) => 8,
        (3, 2) => 6,
        (3, 3) => 4,
        (3, 4) => 2,
        (3, 5) => 2,
        (3, 6) => 3,
        _ => unreachable!()
    }) * match kind {
        1 => 1,
        2 => 10,
        3 => 100,
        4 => 1000,
        _ => unreachable!()
    }
}

// Find all possible new states from the current one.
fn next_states<const N: usize>(s: &State<N>, e: u32) -> Vec<(State<N>, u32)> {
    let mut new = vec![];

    // Two cases for each room:
    for r in 0..4 {
        // Out of the first nonempty slot...
        if let Some(slot) = (0..N).find(|&i| s.rooms[r][i] != 0) {
            let kind = s.rooms[r][slot];
            // ... into any reachable hallway slot in both directions
            let left = (0..r+2).rev().take_while(|&i| s.hall[i] == 0);
            let right = (r+2..7).take_while(|&i| s.hall[i] == 0);
            for hall in left.chain(right) {
                let mut new_s = *s;
                std::mem::swap(&mut new_s.hall[hall], &mut new_s.rooms[r][slot]);
                new.push((new_s, e + energy_cost(kind, r, slot, hall)));
            }
        }
        // Into the furthest empty slot, but only if the room is clear of others...
        let kind = r as u8 + 1;
        if (0..N).all(|i| [0, kind].contains(&s.rooms[r][i])) {
            if let Some(slot) = (0..N).take_while(|&i| s.rooms[r][i] == 0).last() {
                // ... from the first nonempty hallway places in both directions, ...
                let left = (0..r+2).rev().find(|&i| s.hall[i] != 0).into_iter();
                let right = (r+2..7).find(|&i| s.hall[i] != 0);
                // ... but only if the kind matches the room
                for hall in left.chain(right).filter(|&i| s.hall[i] == kind) {
                    let mut new_s = *s;
                    std::mem::swap(&mut new_s.hall[hall], &mut new_s.rooms[r][slot]);
                    new.push((new_s, e + energy_cost(kind, r, slot, hall)));
                }
            }
        }
    }

    new
}

// BFS for the minimum energy cost to get from `init` to `goal`.
fn search<const N: usize>(init: State<N>, goal: State<N>) -> u32 {
    let mut min_e = u32::MAX;
    let mut seen = HashMap::with_capacity(100000);
    let mut queue = HashMap::new();
    queue.insert(init, 0);

    while !queue.is_empty() {
        for (state, cur_e) in std::mem::replace(&mut queue,
                                                HashMap::with_capacity(10000)) {
            for (new_state, new_e) in next_states(&state, cur_e) {
                if new_state == goal {
                    min_e = min_e.min(new_e);
                } else if seen.get(&new_state).map_or(true, |&e| e > new_e) {
                    seen.insert(new_state, new_e);
                    queue.insert(new_state, new_e);
                }
            }
        }
    }
    min_e
}

fn main() {
    let (r1a, r2a, r3a, r4a, r1b, r2b, r3b, r4b) =
        input::chars().filter(|ch| ['A', 'B', 'C', 'D'].contains(ch))
                      .map(|ch| ch as u8 - b'A' + 1).collect_tuple().unwrap();

    // Part 1: rooms with 2 slots.
    let init = State {
        rooms: [[r1a, r1b], [r2a, r2b], [r3a, r3b], [r4a, r4b]],
        hall:  [0; 7],
    };
    let goal = State {
        rooms: [[1, 1], [2, 2], [3, 3], [4, 4]],
        hall:  [0; 7],
    };
    advtools::verify("Two places", search(init, goal), 15365);

    // Part 2: rooms with 4 slots.
    let init = State {
        rooms: [[r1a, 4, 4, r1b], [r2a, 3, 2, r2b], [r3a, 2, 1, r3b], [r4a, 1, 3, r4b]],
        hall:  [0; 7],
    };
    let goal = State {
        rooms: [[1, 1, 1, 1], [2, 2, 2, 2], [3, 3, 3, 3], [4, 4, 4, 4]],
        hall:  [0; 7],
    };
    advtools::verify("Four places", search(init, goal), 52055);
}
