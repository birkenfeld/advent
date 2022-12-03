use advtools::itertools::Itertools;
use advtools::input;

fn main() {
    // Split input by empty lines.
    let chunks = input::string().split("\n\n");

    // Sum all the calories for a single elf, sort the sums descending, and
    // get the highest three.
    let (c1, c2, c3) = chunks.map(|elf| {
        elf.lines().flat_map(str::parse::<i32>).sum::<i32>()
    }).sorted_by_key(|k| -k).next_tuple().unwrap();

    advtools::verify("1-elf maximum", c1, 70698);
    advtools::verify("3-elf maximum", c1 + c2 + c3, 206643);
}
