extern crate itertools;
extern crate advtools;

use itertools::Itertools;

fn find_allowed(excluded: &[(u32, u32)], mut el: u32) -> Option<u32> {
    for &(rmin, rmax) in excluded {
        if rmin <= el && el <= rmax {
            if rmax == u32::max_value() {
                return None;
            }
            el = rmax + 1;
        }
    }
    Some(el)
}

fn main() {
    let excluded = advtools::iter_input::<String>().map(|line| {
        let parts = line.trim().split('-').collect_vec();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    }).sorted();
    let mut smallest = find_allowed(&excluded, 0);
    println!("Smallest allowed: {}", smallest.unwrap());
    let mut n = 0;
    while let Some(el) = smallest {
        smallest = find_allowed(&excluded, el + 1);
        n += 1;
    }
    println!("Number allowed: {}", n);
}
