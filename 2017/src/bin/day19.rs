use advtools::input;
use advtools::grid::{Grid, Dir::*};

fn main() {
    let map = Grid::new(input::raw_string().lines().map(|s| s.as_bytes().to_vec()));

    let mut pos = map.find_pos(|&c| c == b'|').unwrap();
    let mut dir = D;
    let mut path = Vec::new();
    let mut steps = 0;

    loop {
        // One step.
        match map[pos] {
            // Reached a corner: determine whether to step up or down/left or right
            // by checking for empty space.  It is not expected to have a "corner"
            // with two empty spaces next to it.
            b'+' => dir = match dir {
                L | R => if map[pos.up()] != b' ' { U } else { D },
                U | D => if map[pos.left()] != b' ' { L } else { R },
            },
            // Any `|` or `-` just means to go on.
            b'|' | b'-' => {},
            // If we reached a blank with these conditions, we must have reached
            // the end of the path.
            b' ' => break,
            c => path.push(c),
        }
        steps += 1;
        pos.step(dir);
    }

    // Part 1: The letters in order of the path.
    advtools::verify("Path", String::from_utf8_lossy(&path), "LIWQYKMRP");
    // Part 2: The total number of steps taken.
    advtools::verify("Steps", steps, 16764);
}
