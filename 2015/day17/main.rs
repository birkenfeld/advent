extern crate advtools;

const GOAL: i32 = 150;
const GOALCONT: u32 = 4;

fn find(v: &[i32], rest: i32) -> u32 {
    if v.len() == 1 {
        (rest == 0) as u32 + (rest == v[0]) as u32
    } else {
        find(&v[1..], rest) + find(&v[1..], rest - v[0])
    }
}

fn find_with_cont(v: &[i32], rest: i32, ncont: u32) -> u32 {
    if v.len() == 1 {
        (if ncont == 0 && rest == 0 { 1 } else { 0 }) +
            (if ncont == 1 && rest == v[0] { 1 } else { 0 })
    } else {
        find_with_cont(&v[1..], rest, ncont) + find_with_cont(&v[1..], rest - v[0], ncont - 1)
    }
}

fn main() {
    let containers = advtools::iter_input().collect::<Vec<i32>>();
    println!("Combinations: {}", find(&containers, GOAL));
    println!("Combinations with 4 containers: {}",
             find_with_cont(&containers, GOAL, GOALCONT));
}
