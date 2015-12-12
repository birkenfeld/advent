use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let mut nice_rule1 = 0;
    let mut nice_rule2 = 0;
    for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
        let mut prev = '\n';
        let mut pprev = '\n';
        let mut vowels = 0;
        let mut has_double = false;
        let mut naughty = false;
        let mut pairs = HashSet::new();
        let mut has_doublepair = false;
        let mut has_repeated = false;
        let line = line.unwrap();
        for ch in line.chars() {
            if ch == prev {
                has_double = true;
            }
            match ch {
                'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                'b' if prev == 'a' => naughty = true,
                'd' if prev == 'c' => naughty = true,
                'q' if prev == 'p' => naughty = true,
                'y' if prev == 'x' => naughty = true,
                _ => ()
            }
            if !(prev == ch && pprev == prev) {
                if pairs.contains(&(prev, ch)) {
                    has_doublepair = true;
                }
            }
            pairs.insert((prev, ch));
            if ch == pprev {
                has_repeated = true;
            }
            pprev = prev;
            prev = ch;
        }
        if has_double && vowels >= 3 && !naughty {
            nice_rule1 += 1;
        }
        if has_doublepair && has_repeated {
            nice_rule2 += 1;
            println!("{}", line);
        } else if pairs.len() < 15 && has_repeated {
            println!("--------- {} {}", line, pairs.len());
        }
    }
    println!("Nice strings (rule 1): {}", nice_rule1);
    println!("Nice strings (rule 2): {}", nice_rule2);
}
