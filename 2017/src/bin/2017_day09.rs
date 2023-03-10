use advtools::input;

fn main() {
    let mut skip = false;
    let mut garbage = false;
    let mut nesting = 0;
    let mut score = 0;
    let mut ngarbage = 0;
    for ch in input::chars() {
        match (skip, garbage, ch) {
            // The skipped character has the highest priority.
            (true, _, _)   => skip = false,
            // Skipping works in any state.
            (_, _,    '!') => skip = true,
            // In garbage state, ignore anything but closing `>`.
            (_, true, '>') => garbage = false,
            (_, true, _)   => ngarbage += 1,
            // Otherwise, keep track of opening garbage and nesting level.
            (_, _,    '<') => garbage = true,
            (_, _,    '}') => nesting -= 1,
            (_, _,    '{') => { nesting += 1; score += nesting; },
            (_, _,    _)   => (),
        }
    }
    // Part 1: "Score" (sum of nesting levels) of the input.
    advtools::verify("Score", score, 23588);
    // Part 2: Number of characters within garbage.
    advtools::verify("Garbage", ngarbage, 10045);
}
