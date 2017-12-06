extern crate advtools;

use std::collections::HashMap;
use std::mem;

fn main() {
    let mut banks = advtools::iter_input::<Vec<isize>>().next().unwrap();
    let mut seen = HashMap::new();
    let mut steps = 0;
    let len = banks.len();

    let loop_size = loop {
        // Using min because max_by_key() prefers later items with equal key.
        let start_idx = banks.iter().enumerate().min_by_key(|v| -v.1).unwrap().0;
        let n = mem::replace(&mut banks[start_idx], 0);
        for idx in (0..len).cycle().skip(start_idx+1).take(n as usize) {
            banks[idx] += 1;
        }
        steps += 1;
        if let Some(prev) = seen.insert(banks.clone(), steps) {
            break steps - prev;
        }
    };
    println!("Redistribution steps: {}", steps);
    println!("Size of loop: {}", loop_size);
}
