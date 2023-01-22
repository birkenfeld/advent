use advtools::prelude::{Itertools, HashMap, once};
use advtools::input;

const FORMAT: &str = "(.)(.) -> (.)|(.+)";

fn main() {
    let mut rules = HashMap::new();
    let mut iter = input::rx_lines::<(Option<_>, &str)>(FORMAT);
    let initial_poly = iter.next().unwrap().1;

    // Read the rules: construct for each element pair the new pairs that will
    // be generated in the next step.
    for (a, b, c) in iter.filter_map(|line| line.0) {
        rules.insert((a, b), ((a, c), (c, b)));
    }

    // Count the occurrence of element pairs in the initial input.
    let mut pairs = HashMap::<_, usize>::new();
    for (a, b) in initial_poly.chars().tuple_windows() {
        *pairs.entry((a, b)).or_default() += 1;
    }
    let last_el = initial_poly.chars().last().unwrap();

    for step in 1..=40 {
        // Update the counts by consulting the rule for each existing pair.
        let mut new_pairs = HashMap::new();
        for (pair, count) in pairs.iter() {
            let (new_pair1, new_pair2) = rules[pair];
            *new_pairs.entry(new_pair1).or_default() += count;
            *new_pairs.entry(new_pair2).or_default() += count;
        }
        pairs = new_pairs;

        // On steps 10 and 40, stop and count the actual elements.
        if step == 10 || step == 40 {
            // Start with the final element already counted since it is omitted below.
            let mut chars: HashMap<_, _> = once((last_el, 1)).collect();
            // Count the first element of every pair
            for (pair, count) in &pairs {
                *chars.entry(pair.0).or_default() += count;
            }
            // Find min and max counts by character.
            let (least, most) = chars.values().minmax().into_option().unwrap();

            if step == 10 {
                advtools::verify("After 10 steps", most - least, 2360);
            } else {
                advtools::verify("After 40 steps", most - least, 2967977072188usize);
            }
        }
    }
}
