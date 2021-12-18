use advtools::itertools::iterate;
use advtools::input;

fn main() {
    // Let the helper library parse all lines and collect them.
    let masses = input::parse_vec::<i64>();
    // Just a normal iteration with map() to calculate the total.
    let total: i64 = masses.iter().map(|m| m/3 - 2).sum();
    advtools::verify("Fuel for modules", total, 3159380);

    // For part 2, create a subiterator for each mass, and sum up all
    // the masses.  iterate() applies the function over and over.
    let total_2: i64 = masses.iter().flat_map(
        |&m| iterate(m, |&m| m/3 - 2).skip(1).take_while(|&m| m > 0)
    ).sum();
    advtools::verify("Total fuel", total_2, 4736213);
}
