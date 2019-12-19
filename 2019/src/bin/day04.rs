use std::iter::once;
use advtools::prelude::Itertools;
use advtools::input::to_u32;

const INPUT: &str = "136760-595730";

fn match1(s: &String) -> bool {
    // Test if the digits (chars) are sorted
    s.chars().tuple_windows().all(|(c1, c2)| c1 <= c2) &&
        // and if there is any consecutive equal pair.
        s.chars().tuple_windows().any(|(c1, c2)| c1 == c2)
}

fn match2(s: &String) -> bool {
    // Test if in any group of 4 digits, only the two middle ones match.
    // To avoid special casing the beginning and end, add non-participating
    // characters there.
    once('_').chain(s.chars()).chain(once('_')).tuple_windows().any(
        |(c1, c2, c3, c4)| c1 != c2 && c2 == c3 && c4 != c2
    )
}

fn main() {
    let (min, max) = INPUT.split('-').map(to_u32).next_tuple().unwrap();

    // Make two identical iterators over all numbers matching condition 1.
    let (i1, i2) = (min..=max).map(|x| x.to_string()).filter(match1).tee();

    // For solutions, count all numbers, or only those matching condition 2.
    advtools::print("First round", i1.count());
    advtools::print("Second round", i2.filter(match2).count());
}
