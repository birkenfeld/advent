use advtools::prelude::Itertools;
use advtools::input;

fn push_pair(v: &mut Vec<u8>, n: u8, d: u8) {
    if n >= 100 {
        v.push(n / 100);
        v.push((n % 100) / 10);
        v.push(n % 10);
    } else if n >= 10 {
        v.push(n / 10);
        v.push(n % 10);
    } else {
        v.push(n);
    }
    v.push(d);
}

fn main() {
    let mut seq = input::chars().map(|ch| ch as u8 - b'0').collect_vec();
    for i in 1..=50 {
        let mut new_seq = Vec::with_capacity(2 * seq.len());
        let mut dp = seq[0];
        let mut n = 0;
        for d in seq {
            if d != dp {
                push_pair(&mut new_seq, n, dp);
                n = 0;
            }
            n += 1;
            dp = d;
        }
        push_pair(&mut new_seq, n, dp);
        seq = new_seq;
        if i == 40 {
            advtools::verify("Resulting length after 40 iterations", seq.len(), 492982);
        } else if i == 50 {
            advtools::verify("Resulting length after 50 iterations", seq.len(), 6989950);
        }
    }
}
