use advtools::itertools::Itertools;
use advtools::input;

fn main() {
    // Split input by empty lines.
    let chunks = input::string().split("\n\n");

    // Sum all the calories for a single elf, and sort the sums (descending).
    let all_cals = chunks.map(|elf| {
        elf.split('\n').map(|item| item.parse::<i32>().unwrap()).sum::<i32>()
    }).sorted_by_key(|k| -k).collect_vec();

    advtools::verify("1-elf maximum", all_cals[0], 70698);
    advtools::verify("3-elf maximum", all_cals[0..3].iter().sum::<i32>(), 206643);
}
