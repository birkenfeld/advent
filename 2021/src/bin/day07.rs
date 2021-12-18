use advtools::prelude::Itertools;
use advtools::input;

// Calculates the min fuel for a given fuel cost function.
fn min_fuel(pos: &[i32], fuel: impl Fn(i32) -> i32) -> i32 {
    // Get the min/max candidate positions.
    let (minpos, maxpos) = pos.iter().copied().minmax().into_option().unwrap();
    (minpos..=maxpos).map(|q| {
        pos.iter().map(|p| fuel((p - q).abs())).sum::<i32>()
    }).min().unwrap()
}

fn main() {
    let pos = input::string().split(',').map(input::to_i32).collect_vec();

    advtools::verify("Linear fuel", min_fuel(&pos, |d| d), 336120);
    advtools::verify("Quadratic fuel", min_fuel(&pos, |d| d*(d+1) / 2), 96864235);
}
