use advtools::input;
use advtools::prelude::Itertools;

// Predict the previous value in the sequence.
fn predict(seq: Vec<i64>) -> i64 {
    let diffs = seq.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();
    seq[0] - if diffs.iter().all_equal() {
        diffs[0]
    } else {
        predict(diffs)
    }
}

fn main() {
    let seqs = input::lines().map(
        |line| line.split_whitespace().map(|n| n.parse().unwrap()).collect_vec()
    ).collect_vec();

    // Part 1: predict next value, i.e. reverse all sequences.
    let part1 = seqs.iter().map(|seq| {
        predict(seq.iter().cloned().rev().collect())
    }).sum::<i64>();
    advtools::verify("Sum of next values", part1, 2043183816);

    // Part 2: predict previous value.
    let part2 = seqs.into_iter().map(predict).sum::<i64>();
    advtools::verify("Sum of previous values", part2, 1118);
}
