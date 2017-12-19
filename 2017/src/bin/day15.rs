extern crate advtools;
use advtools::prelude::*;

const START_A:  u64 = 699;
const FACTOR_A: u64 = 16807;
const START_B:  u64 = 124;
const FACTOR_B: u64 = 48271;

fn mult_mod(a: &u64, b: u64) -> u64 {
    // http://www.firstpr.com.au/dsp/rand31/#Simpler
    let prod = a * b;
    let res = (prod >> 31) + (prod & 0x7FFF_FFFF);
    (res & 0x7FFF_FFFF) + (res >> 31)
}

fn compare_seqs<FA, FB>(n: usize, accept_a: FA, accept_b: FB) -> usize
    where FA: Fn(&u64) -> bool, FB: Fn(&u64) -> bool
{
    let ia = itertools::iterate(START_A, |a| mult_mod(a, FACTOR_A)).filter(accept_a);
    let ib = itertools::iterate(START_B, |b| mult_mod(b, FACTOR_B)).filter(accept_b);
    ia.zip(ib).take(n).filter(|&(a, b)| a as u16 == b as u16).count()
}

fn main() {
    println!("Accepted #1: {}", compare_seqs(40_000_000, |_| true, |_| true));
    println!("Accepted #2: {}", compare_seqs(5_000_000, |a| a & 3 == 0, |b| b & 7 == 0));
}
