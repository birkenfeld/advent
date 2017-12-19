extern crate advtools;
extern crate rayon;

use advtools::prelude::*;
use rayon::prelude::*;

fn main() {
    let firewall: HashMap<i32, _> = iter_input_trim(":").collect();

    let severity = firewall.iter().map(|(depth, range)| {
        if depth % (2*range - 2) == 0 { range * depth } else { 0 }
    }).sum::<i32>();
    println!("Severity: {}", severity);

    let delay = (0..10_000_000).into_par_iter().find_first(|delay| {
        firewall.iter().all(|(depth, range)| (depth + delay) % (2*range - 2) != 0)
    }).unwrap();
    println!("Delay without getting caught: {}", delay);
}
