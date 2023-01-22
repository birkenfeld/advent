use advtools::prelude::Itertools;
use advtools::input;

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
    let excluded = input::rx_lines(r"(\d+)-(\d+)").sorted().collect_vec();
    let mut smallest = find_allowed(&excluded, 0);
    advtools::verify("Smallest allowed", smallest.unwrap(), 17348574);

    let mut n = 0;
    while let Some(el) = smallest {
        smallest = find_allowed(&excluded, el + 1);
        n += 1;
    }
    advtools::verify("Number allowed", n, 104);
}
