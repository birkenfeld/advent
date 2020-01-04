use advtools::prelude::Itertools;
use advtools::input::iter_input_regex;
use std::cell::Cell;

fn find_bridges(parts: &[(u32, u32, Cell<bool>)], connect: u32, mut strength: u32, mut length: u32,
                strongest: &mut u32, longest: &mut (u32, u32)) {
    for &(conn_a, conn_b, ref used) in parts {
        if (conn_a == connect || conn_b == connect) && !used.get() {
            let new_connect = if conn_a == connect { conn_b } else { conn_a };
            // Mark part as used and update our state.
            used.set(true);
            length += 1;
            strength += conn_a + conn_b;
            // Check if we have a new strength/length record.
            // `longest` as a tuple automatically implements the right ordering relation
            // (compare length first, then strength).
            *strongest = strength.max(*strongest);
            *longest = (length, strength).max(*longest);
            find_bridges(parts, new_connect, strength, length, strongest, longest);
            // Restore previous state for trying the next part.
            strength -= conn_a + conn_b;
            length -= 1;
            used.set(false);
        }
    }
}

fn main() {
    let parts = iter_input_regex("(\\d+)/(\\d+)")
        .map(|(a, b)| (a, b, Cell::default())).collect_vec();

    let mut strongest = 0;
    let mut longest = (0, 0);
    // Go through all bridge combinations recursively, using DFS.
    find_bridges(&parts, 0, 0, 0, &mut strongest, &mut longest);
    // Part 1: Find maximum strength of any bridge.
    advtools::verify("Max bridge strength", strongest, 1511);
    // Part 2: Find maximum strength of longest bridges.
    advtools::verify("Longest bridge strength", longest.1, 1471);
}
