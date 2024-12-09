use advtools::input;
use advtools::prelude::HashMap;

fn main() {
    let mut first = Vec::new();
    let mut second = Vec::new();
    // counts of second-column numbers for part 2
    let mut second_count = HashMap::new();
    input::parse_lines().for_each(|nums: (i64, i64)| {
        first.push(nums.0);
        second.push(nums.1);
        *second_count.entry(nums.1).or_default() += 1;
    });
    first.sort();
    second.sort();

    // part 1: just add up absolute differences
    let sum_diff = first.iter().zip(&second).map(|(a, &b)| a.abs_diff(b)).sum::<u64>();
    advtools::verify("Sum of differences", sum_diff, 2430334);

    // part 2: multiply each first-column item with the count in the second
    let sim_score = first.iter().map(|a| second_count.get(a).unwrap_or(&0) * a).sum::<i64>();
    advtools::verify("Similarity score", sim_score, 28786472);
}
