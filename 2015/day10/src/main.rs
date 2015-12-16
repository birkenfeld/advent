use std::fmt::Write;

const INPUT: &'static str = "1321131112";

fn main() {
    let mut seq = String::from(INPUT);
    for i in 1..51 {
        let mut new_seq = String::new();
        let mut pch = '\n';
        let mut run = 0;
        for ch in seq.chars() {
            if ch != pch && pch != '\n' {
                write!(new_seq, "{}", run).unwrap();
                new_seq.push(pch);
                run = 0;
            }
            run += 1;
            pch = ch;
        }
        write!(new_seq, "{}", run).unwrap();
        new_seq.push(pch);
        seq = new_seq;
        if i > 35 && i % 10 == 0 {
            println!("Resulting length after {} iterations: {}", i, seq.len());
        }
    }
}
