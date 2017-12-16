const START_A:  u64 = 699;
const FACTOR_A: u64 = 16807;
const START_B:  u64 = 124;
const FACTOR_B: u64 = 48271;
const MODULUS:  u64 = (1 << 31) - 1;

fn compare_seqs<FA, FB>(n: u64, accept_a: FA, accept_b: FB) -> usize
    where FA: Fn(u64) -> bool, FB: Fn(u64) -> bool
{
    (0..n).scan((START_A, START_B), |vals, _| {
        loop {
            vals.0 = (vals.0 * FACTOR_A) % MODULUS;
            if accept_a(vals.0) { break; }
        }
        loop {
            vals.1 = (vals.1 * FACTOR_B) % MODULUS;
            if accept_b(vals.1) { break; }
        }
        Some(vals.0 & 0xFFFF == vals.1 & 0xFFFF)
    }).filter(|&b| b).count()
}

fn main() {
    println!("Accepted #1: {}", compare_seqs(40_000_000, |_| true, |_| true));
    println!("Accepted #2: {}", compare_seqs(5_000_000,
                                             |va| va & 3 == 0,
                                             |vb| vb & 7 == 0));
}
