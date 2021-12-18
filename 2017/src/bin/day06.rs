use advtools::prelude::HashMap;
use advtools::input;

fn main() {
    let mut banks = input::parse::<Vec<i32>>();
    let mut seen = HashMap::new();
    let mut steps = 0;
    let len = banks.len();

    let loop_size = loop {
        // Find the bank with the maximum number of allocations.
        // Using `min` and `-` because max_by_key() prefers later items with equal key.
        let start_idx = banks.iter().enumerate().min_by_key(|v| -v.1).unwrap().0;
        let n = std::mem::replace(&mut banks[start_idx], 0);
        // Redistribute "n" over all banks, starting with the next one.
        for idx in (0..len).cycle().skip(start_idx+1).take(n as usize) {
            banks[idx] += 1;
        }
        steps += 1;
        // Exit condition: configuration was already seen.
        if let Some(prev) = seen.insert(banks.clone(), steps) {
            break steps - prev;
        }
    };
    advtools::verify("Redistribution steps", steps, 11137);
    advtools::verify("Size of loop", loop_size, 1037);
}
