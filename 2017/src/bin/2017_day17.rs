use advtools::input;

fn main() {
    let skip: u32 = input::parse();

    // Part 1: insert values 2018 times as directed.
    let mut buf = vec![0];
    let mut pos = 0;
    for n in 1..=2017 {
        pos = (pos + skip) % n + 1;
        if pos == n {
            buf.push(n);
        } else {
            buf.insert(pos as usize, n);
        }
    }
    let after_pos = if pos < 2017 { pos + 1 } else { 0 };
    advtools::verify("After 2017", buf[after_pos as usize], 640);

    // Part 2: don't actually keep track of the inserted values.  Since we're
    // looking for the value after "0", and "0" always stays at the front of the
    // list, only keep track of the insert position and determine which numbers
    // are inserted as position 1.
    let mut pos = 0;
    let mut after_zero = 0;
    for n in 1..50_000_000 {
        pos += skip;
        // Optimization: avoid calculating the modulo if unnecessary.
        if pos >= n {
            pos %= n;
            if pos == 0 {
                after_zero = n;
            }
        }
        pos += 1;
    }
    advtools::verify("After zero", after_zero, 47949463);
}
