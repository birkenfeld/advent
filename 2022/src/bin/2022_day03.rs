use advtools::input;
use advtools::itertools::Itertools;

/// Find the priority of the common element of all given slices.
fn common_prio(itemlists: &[impl AsRef<[u8]>]) -> i32 {
    // For the small slice size, iteration is faster than conversion to sets.
    let common = itemlists[0].as_ref().iter().find(
        |item| itemlists[1..].iter().all(|l| l.as_ref().contains(item))
    );
    match common {
        Some(c @ b'a'..=b'z') => (c - b'a' + 1) as i32,
        Some(c @ b'A'..=b'Z') => (c - b'A' + 27) as i32,
        _ => unreachable!()
    }
}

fn main() {
    // Part 1: split each line in half.
    let score = input::lines().map(|line| {
        let line = line.as_bytes();
        common_prio(&[&line[..line.len()/2], &line[line.len()/2..]])
    }).sum::<i32>();
    advtools::verify("Common score", score, 7817);

    // Part 2: go through lines in chunks of 3. (Todo: Use array_chunks once stable.)
    let score = input::lines().tuples().map(|(line1, line2, line3)| {
        common_prio(&[line1, line2, line3])
    }).sum::<i32>();
    advtools::verify("Badge score", score, 2444);
}
