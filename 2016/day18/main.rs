extern crate advtools;
extern crate extprim;

use extprim::u128::u128;

fn count_all_safe(n: usize) -> usize {
    let u1 = u128::one();

    // read from input: line = 0...input...0
    let mut line = u128::zero();
    let len = advtools::input_string().trim().chars().map(|ch|
        line = (if ch == '^' { line | u1 } else { line }) << 1
    ).count();
    // mask = 0111...1110
    let mask = ((u1 << len) - u1) << 1;

    let mut count = 0;
    for _ in 0..n {
        count += len - line.count_ones() as usize;
        // new line: only the tiles right and left matter, consider 10111:
        // 10111.   shifted left
        //  .10111  shifted right
        // -------
        // 1001011  XORed
        // .00101.  with mask applied
        line = ((line << 1) ^ (line >> 1)) & mask;
    }
    count
}

fn main() {
    println!("Safe tiles (40 lines): {}", count_all_safe(40));
    println!("Safe tiles (400000 lines): {}", count_all_safe(400000));
}
