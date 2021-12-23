use advtools::prelude::{Itertools, HashMap};
use advtools::input;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct State<const N: usize> {
    rooms: [[u8; N]; 4],
    hall:  [u8; 11],
}

// Calculate energy cost between a `slot` (in the given `room`) and a `hall` place.
fn energy_cost(kind: u8, room: usize, slot: usize, hall: usize) -> u32 {
    let room_hall = 2 + (room * 2);
    let hall_steps = if room_hall < hall { hall - room_hall } else { room_hall - hall };
    let steps = slot + 1 + hall_steps;
    steps as u32 * match kind {
        1 => 1,
        2 => 10,
        3 => 100,
        4 => 1000,
        _ => unreachable!()
    }
}

// Find all possible new states from the current one.
fn next_states<const N: usize>(state: &State<N>, e: u32) -> Vec<(State<N>, u32)> {
    let State { rooms, hall } = state;
    let mut states = vec![];

    // Check possible moves from/to each room.
    for (r, room) in rooms.iter().enumerate() {
        let room_kind = r as u8 + 1;
        let room_hall = 2 + r*2;
        let mut left_hall = (0..room_hall).rev();
        let mut right_hall = room_hall + 1..hall.len();

        if (0..N).all(|i| [0, room_kind].contains(&room[i])) {
            // If the room is already filled with only correct kinds,
            // and has a free slot, consider moving into it ...
            if let Some(s) = (0..N).take_while(|&s| room[s] == 0).last() {
                // ... from the first nonempty hallway slot in both directions ...
                let left = left_hall.find(|&h| hall[h] != 0).into_iter();
                let right = right_hall.find(|&h| hall[h] != 0);
                // ... but only if the kind matches the room.
                for h in left.chain(right).filter(|&h| hall[h] == room_kind) {
                    let mut new = *state;
                    std::mem::swap(&mut new.hall[h], &mut new.rooms[r][s]);
                    states.push((new, e + energy_cost(room_kind, r, s, h)));
                }
            }
        } else {
            // Otherwise, consider all moves of the topmost filled room slot ...
            if let Some(s) = (0..N).find(|&s| room[s] != 0) {
                // ... into any reachable hallway slot in both directions ...
                let left = left_hall.take_while(|&h| hall[h] == 0);
                let right = right_hall.take_while(|&h| hall[h] == 0);
                // ... which is not in front of a room (2, 4, 6, 8).
                for h in left.chain(right).filter(|h| ![2, 4, 6, 8].contains(h)) {
                    let mut new = *state;
                    std::mem::swap(&mut new.hall[h], &mut new.rooms[r][s]);
                    states.push((new, e + energy_cost(room[s], r, s, h)));
                }
            }
        }
    }

    states
}

// BFS for the minimum energy cost to get from `init` to `goal`.
fn search<const N: usize>(init: State<N>, goal: State<N>) -> u32 {
    let mut min_e = u32::MAX;
    let mut seen = HashMap::with_capacity(100000);
    let mut todo = HashMap::new();
    todo.insert(init, 0);

    while !todo.is_empty() {
        for (state, cur_e) in std::mem::replace(&mut todo,
                                                HashMap::with_capacity(10000)) {
            for (new_state, new_e) in next_states(&state, cur_e) {
                if new_state == goal {
                    min_e = min_e.min(new_e);
                } else if seen.get(&new_state).map_or(true, |&e| e > new_e) {
                    seen.insert(new_state, new_e);
                    todo.insert(new_state, new_e);
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
        hall:  [0; 11],
    };
    let goal = State {
        rooms: [[1, 1], [2, 2], [3, 3], [4, 4]],
        hall:  [0; 11],
    };
    advtools::verify("Two places", search(init, goal), 15365);

    // Part 2: rooms with 4 slots.
    let init = State {
        rooms: [[r1a, 4, 4, r1b], [r2a, 3, 2, r2b], [r3a, 2, 1, r3b], [r4a, 1, 3, r4b]],
        hall:  [0; 11],
    };
    let goal = State {
        rooms: [[1, 1, 1, 1], [2, 2, 2, 2], [3, 3, 3, 3], [4, 4, 4, 4]],
        hall:  [0; 11],
    };
    advtools::verify("Four places", search(init, goal), 52055);
}
