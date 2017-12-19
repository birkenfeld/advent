const SKIP: u32 = 349;

fn main() {
    let mut buf = vec![0];
    let mut pos = 0;
    for n in 1..2018 {
        pos = (pos + SKIP) % n + 1;
        if pos == n {
            buf.push(n);
        } else {
            buf.insert(pos as usize, n);
        }
    }
    let after_pos = if pos < 2017 { pos + 1 } else { 0 };
    println!("After 2017: {}", buf[after_pos as usize]);

    let mut pos = 0;
    let mut after_zero = 0;
    for n in 1..50_000_000 {
        pos += SKIP;
        if pos >= n {
            pos %= n;
            if pos == 0 {
                after_zero = n;
            }
        }
        pos += 1;
    }
    println!("After zero: {}", after_zero);
}
