use advtools::input;
use advtools::rayon::prelude::*;

fn main() {
    let firewall = input::parse_vec::<(i32, i32)>();

    // Part 1: Evaluate severity, determined by Sum(range*depth).
    let severity = firewall.iter().map(|(depth, range)| {
        if depth % (2*range - 2) == 0 { range * depth } else { 0 }
    }).sum::<i32>();
    advtools::verify("Severity", severity, 2160);

    // Part 2: Find time offset to pass through the firewall uncaught.
    // This does not correspond to severity == 0, since getting caught at depth 0
    // still counts as getting caught!
    let delay = (0..10_000_000).into_par_iter().find_first(|delay| {
        firewall.iter().all(|(depth, range)| (depth + delay) % (2*range - 2) != 0)
    }).unwrap();
    advtools::verify("Delay without getting caught", delay, 3907470);
}
