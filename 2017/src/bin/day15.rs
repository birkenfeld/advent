const START_A:  u64 = 699;
const FACTOR_A: u64 = 16807;
const START_B:  u64 = 124;
const FACTOR_B: u64 = 48271;

fn mult_and_check<F: Fn(u64) -> bool>(a: &mut u64, b: u64, accept: F) -> bool {
    // http://www.firstpr.com.au/dsp/rand31/#Simpler
    let prod = *a * b;
    let res = (prod >> 31) + (prod & 0x7FFF_FFFF);
    *a = (res & 0x7FFF_FFFF) + (res >> 31);
    accept(*a)
}

fn compare_seqs<FA, FB>(n: u32, accept_a: FA, accept_b: FB) -> usize
    where FA: Fn(u64) -> bool, FB: Fn(u64) -> bool
{
    let (mut a, mut b) = (START_A, START_B);
    (0..n).filter(|_| {
        while !mult_and_check(&mut a, FACTOR_A, &accept_a) { }
        while !mult_and_check(&mut b, FACTOR_B, &accept_b) { }
        a & 0xFFFF == b & 0xFFFF
    }).count()
}

fn main() {
    println!("Accepted #1: {}", compare_seqs(40_000_000, |_| true, |_| true));
    println!("Accepted #2: {}", compare_seqs(5_000_000, |a| a & 3 == 0, |b| b & 7 == 0));
}
