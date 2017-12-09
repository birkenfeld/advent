extern crate advtools;

fn main() {
    let mut skip = false;
    let mut garbage = false;
    let mut nesting = 0;
    let mut score = 0;
    let mut ngarbage = 0;
    for ch in advtools::input_string().chars() {
        match (skip, garbage, ch) {
            (true, _, _)   => skip = false,
            (_, _,    '!') => skip = true,
            (_, true, '>') => garbage = false,
            (_, true, _)   => ngarbage += 1,
            (_, _,    '<') => garbage = true,
            (_, _,    '}') => nesting -= 1,
            (_, _,    '{') => { nesting += 1; score += nesting; },
            (_, _,    _)   => (),
        }
    }
    println!("Score: {}", score);
    println!("Garbage: {}", ngarbage);
}
