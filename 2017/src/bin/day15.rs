use advtools::prelude::{Itertools, itertools::iterate};
use advtools::input::iter_input;

/// Calculate a*b (mod 2^31-1) without doing a `%` operation, algorithm taken
/// from http://www.firstpr.com.au/dsp/rand31/#Simpler.
fn mult_mod(a: &u64, b: u64) -> u64 {
    let prod = a * b;
    let res = (prod >> 31) + (prod & 0x7FFF_FFFF);
    (res & 0x7FFF_FFFF) + (res >> 31)
}

/// Implements the judging process with closures determining which generated
/// numbers are accepted for comparison.
fn compare_seqs<FA, FB>(
    n: usize, accept_a: FA, accept_b: FB,
    (start_a, factor_a, start_b, factor_b): (u64, u64, u64, u64)) -> usize
where FA: Fn(&u64) -> bool, FB: Fn(&u64) -> bool
{
    let ia = iterate(start_a, |a| mult_mod(a, factor_a)).filter(accept_a);
    let ib = iterate(start_b, |b| mult_mod(b, factor_b)).filter(accept_b);
    ia.zip(ib).take(n).filter(|&(a, b)| a as u16 == b as u16).count()
}

fn main() {
    let input = iter_input().collect_tuple().unwrap();

    // Part 1: 40m numbers, all are accepted.
    advtools::print("Accepted #1", compare_seqs(40_000_000, |_| true, |_| true, input));
    // Part 2: 5m numbers, only accept divisible by 4 / 8, respectively.
    advtools::print("Accepted #2", compare_seqs(5_000_000, |a| a & 3 == 0, |b| b & 7 == 0, input));
}
