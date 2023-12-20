use advtools::input;
use advtools::prelude::{HashMap, lcm};

// Walk the graph until an end position matching the condition is reached.
fn walk<'a>(path: &str, maze: &HashMap<&'a str, (&'a str, &'a str)>,
            mut pos: &'a str, end: impl Fn(&str) -> bool) -> u64 {
    let mut steps = path.chars().cycle();
    let mut num = 0;
    loop {
        pos = match steps.next() {
            Some('L') => maze[pos].0,
            Some('R') => maze[pos].1,
            _ => unreachable!()
        };
        num += 1;
        if end(pos) {
            return num;
        }
    }
}

fn main() {
    let path = input::lines().next().unwrap();
    let maze = input::rx_lines::<(&str, (&str, &str))>(r"(...) = \((...), (...)\)|.*")
        .collect::<HashMap<_, _>>();

    // Part 1: just walk from AAA to ZZZ.
    let simple = walk(path, &maze, "AAA", |pos| pos == "ZZZ");
    advtools::verify("Steps to ZZZ", simple, 15989);

    // Part 2: determine cycle lengths starting at all positions ending with A,
    // going to positions in Z.
    let cycles = maze.keys().filter(|n| n.ends_with('A'))
        .map(|&from| walk(path, &maze, from, |pos| pos.ends_with('Z')))
        .collect::<Vec<_>>();
    // The total cycle is the LCM of all such cycles.
    let min_steps = cycles.into_iter().reduce(lcm).unwrap();
    advtools::verify("Ghost steps", min_steps, 13830919117339u64);
}
