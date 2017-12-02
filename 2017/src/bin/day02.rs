extern crate advtools;
extern crate itertools;

use itertools::Itertools;

fn main() {
    let input = advtools::iter_input::<Vec<i32>>().collect_vec();
    let cksum1 = input.iter().map(|cols| cols.iter().minmax().into_option().unwrap())
                             .map(|(&min, &max)| max - min)
                             .sum::<i32>();
    let div = |a: i32, b: i32| {
        if a % b == 0 { Some(a / b) }
        else if b % a == 0 { Some(b / a) }
        else { None }
    };
    let cksum2 = input.iter().map(|cols| {
        cols.iter().tuple_combinations().filter_map(|(&a, &b)| div(a, b)).next().unwrap()
    }).sum::<i32>();
    println!("Checksum 1: {}", cksum1);
    println!("Checksum 2: {}", cksum2);
}
