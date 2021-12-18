use advtools::input;

fn main() {
    // Collect initial age timers.
    let mut ages = vec![0u64; 9];
    for age in input::string().split(',').map(input::to_usize) {
        ages[age] += 1;
    }

    for gen in 1..=256 {
        // Update the counts for each age timer.
        ages = (0..9).map(|i| ages[(i + 1) % 9]).collect();
        // The zero timers rolled over to 8 (newborns).  Add the
        // parent fish again at their new timer value 6.
        ages[6] += ages[8];

        if gen == 80 {
            advtools::verify("After 80 days", ages.iter().sum::<u64>(), 350605);
        }
    }

    advtools::verify("After 256 days", ages.iter().sum::<u64>(), 1592778185024u64);
}
