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
    let mut zero_pos = 0;
    let mut after_zero = 0;
    for n in 1..50_000_000 {
        pos = (pos + SKIP) % n + 1;
        if pos <= zero_pos {
            zero_pos += 1;
        } else if pos == zero_pos + 1 {
            after_zero = n;
        }
    }
    println!("After zero: {}", after_zero);
}
