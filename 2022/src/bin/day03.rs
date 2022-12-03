use advtools::itertools::Itertools;
use advtools::prelude::HashSet;
use advtools::input;

fn common_prio(items: &[impl AsRef<[u8]>]) -> i32 {
    let sets = items.iter().map(|r| r.as_ref().iter().collect::<HashSet<_>>());
    match sets.reduce(|a, b| (&a) & (&b)).into_iter().flatten().next() {
        Some(c @ b'a'..=b'z') => (c - b'a' + 1) as i32,
        Some(c @ b'A'..=b'Z') => (c - b'A' + 27) as i32,
        _ => unreachable!()
    }
}

fn main() {
    let score = input::lines().map(|line| {
        let line = line.as_bytes();
        common_prio(&[&line[..line.len()/2], &line[line.len()/2..]])
    }).sum::<i32>();
    advtools::verify("Common score", score, 7817);

    let score = input::lines().tuples().map(|(line1, line2, line3)| {
        common_prio(&[line1, line2, line3])
    }).sum::<i32>();
    advtools::verify("Badge score", score, 2444);
}
