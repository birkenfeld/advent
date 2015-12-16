const INPUT: &'static str = "1321131112";

fn push_pair(v: &mut Vec<u8>, n: u8, d: u8) {
    if n >= 10 {
        for ch in format!("{}", n).chars() {
            v.push(ch.to_digit(10).unwrap() as u8);
        }
    } else {
        v.push(n);
    }
    v.push(d);
}

fn main() {
    let mut seq: Vec<_> = INPUT.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();
    for i in 1..51 {
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
        if i > 35 && i % 10 == 0 {
            println!("Resulting length after {} iterations: {}", i, seq.len());
        }
    }
}
