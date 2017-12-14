extern crate advtools;
use advtools::prelude::*;

fn main() {
    let firewall: HashMap<i32, _> = iter_input_trim(":").collect();

    let severity = firewall.iter().map(|(depth, range)| {
        if depth % (2*range - 2) == 0 { range * depth } else { 0 }
    }).sum::<i32>();
    println!("Severity: {}", severity);

    for delay in 0.. {
        if firewall.iter().all(|(depth, range)| (depth + delay) % (2*range - 2) != 0)  {
            println!("Delay without getting caught: {}", delay);
            break;
        }
    }
}
