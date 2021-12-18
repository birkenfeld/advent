use advtools::prelude::Itertools;
use advtools::input;

fn main() {
    let input = input::string().split(',').map(input::to_u8).collect_vec();

    // Part 1: Preliminary knot hash.
    let marks = advent17::knot_process(&input, 1);
    advtools::verify("Product of first elements", (marks[0] as u16) * (marks[1] as u16), 23715);

    // Part 2: Full knot hash of the input.
    let hash = advent17::knot_hash(input::string());
    advtools::verify("Knot hash", format!("{:02x}", hash.into_iter().format("")),
                     "541dc3180fd4b72881e39cf925a50253");
}
