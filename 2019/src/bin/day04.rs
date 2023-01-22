use std::iter::once;
use advtools::prelude::Itertools;
use advtools::rayon::prelude::*;
use advtools::input;

fn match1(s: &String) -> bool {
    // Test if the digits (chars) are sorted
    s.chars().tuple_windows().all(|(c1, c2)| c1 <= c2) &&
        // and if there is any consecutive equal pair.
        s.chars().tuple_windows().any(|(c1, c2)| c1 == c2)
}

fn match2(s: &&String) -> bool {
    // Test if in any group of 4 digits, only the two middle ones match.
    // To avoid special casing the beginning and end, add non-participating
    // characters there.
    once('_').chain(s.chars()).chain(once('_')).tuple_windows().any(
        |(c1, c2, c3, c4)| c1 != c2 && c2 == c3 && c4 != c2
    )
}

fn main() {
    let (min, max) = input::rx_parse::<(u32, u32)>(r"(\d+)-(\d+)");

    // Find all numbers matching the first condition.
    let first: Vec<_> = (min..=max).into_par_iter().map(|x| x.to_string())
                                                   .filter(match1).collect();
    advtools::verify("Matching first criteria", first.len(), 1873);

    // Now count only those also matching the second condition.
    advtools::verify("Matching also second criteria",
                    first.iter().filter(match2).count(), 1264);
}
