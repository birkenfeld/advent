use advtools::prelude::Itertools;
use advtools::input::{input_string, from_utf8};

enum Dir { U, D, L, R }
use self::Dir::*;

fn main() {
    let map = input_string().lines().map(|s| s.as_bytes().to_vec()).collect_vec();

    let mut x = map[0].iter().position(|&c| c == b'|').unwrap();
    let mut y = 0;
    let mut dir = D;
    let mut path = Vec::new();
    let mut steps = 0;

    loop {
        // One step.
        match map[y][x] {
            // Reached a corner: determine whether to step up or down/left or right
            // by checking for empty space.  It is not expected to have a "corner"
            // with two empty spaces next to it.
            b'+' => dir = match dir {
                L | R => if map[y-1][x] != b' ' { U } else { D },
                U | D => if map[y][x-1] != b' ' { L } else { R },
            },
            // Any `|` or `-` just means to go on.
            b'|' | b'-' => {},
            // If we reached a blank with these conditions, we must have reached
            // the end of the path.
            b' ' => break,
            c => path.push(c),
        }
        steps += 1;
        match dir {
            U => y -= 1,
            D => y += 1,
            L => x -= 1,
            R => x += 1,
        }
    }

    // Part 1: The letters in order of the path.
    advtools::print("Path", from_utf8(path));
    // Part 2: The total number of steps taken.
    advtools::print("Steps", steps);
}
