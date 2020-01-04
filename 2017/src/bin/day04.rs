use advtools::prelude::Itertools;
use advtools::input::iter_input;

fn main() {
    let phrases = iter_input::<Vec<String>>().collect_vec();

    // Part 1: Check if number of unique words == number of words.
    let count1 = phrases.iter().filter(|ph| {
        ph.len() == ph.iter().unique().count()
    }).count();
    advtools::verify("Valid passphrases", count1, 466);

    // Part 2: Check uniqueness after sorting, which considers anagrams.
    let count2 = phrases.iter().filter(|ph| {
        ph.len() == ph.iter().unique_by(|w| w.chars().sorted().collect_vec()).count()
    }).count();
    advtools::verify("Valid without anagrams", count2, 251);
}
