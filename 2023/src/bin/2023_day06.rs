use advtools::input;
use advtools::prelude::Itertools;

fn main() {
    let ways = |(time, record): (f64, f64)| {
        let det = (time.powf(2.) - 4.*record).sqrt();
        ((0.5 * (time + det)).floor() - (0.5 * (time - det)).ceil()) as u64 + 1
    };

    // Part 1: separate races.
    let (times, dist) = input::lines().map(|line| {
        line.split_whitespace().skip(1).map(|v| v.parse::<f64>().unwrap()).collect_vec()
    }).collect_tuple().unwrap();
    let product = times.into_iter().zip(dist).map(ways).product::<u64>();
    advtools::verify("Product of races", product, 6209190);

    // Part 2: only one race.
    let single = input::lines().map(|line| {
        line.split_once(":").unwrap().1.replace(" ", "").parse::<f64>().unwrap()
    }).collect_tuple().unwrap();
    advtools::verify("Only one race", ways(single), 28545089);
}
