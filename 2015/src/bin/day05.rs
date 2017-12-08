extern crate advtools;
use advtools::prelude::*;

fn main() {
    let mut nice_rule1 = 0;
    let mut nice_rule2 = 0;
    for line in iter_input::<String>() {
        let mut prev = '\n';
        let mut pprev = '\n';
        let mut vowels = 0;
        let mut has_double = false;
        let mut has_naughty = false;
        let mut pairs = HashMap::new();
        let mut has_doublepair = false;
        let mut has_repeated = false;
        for (i, ch) in line.chars().enumerate() {
            if ch == prev {
                has_double = true;
            }
            if ch == pprev {
                has_repeated = true;
            }
            match ch {
                'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                'b' if prev == 'a' => has_naughty = true,
                'd' if prev == 'c' => has_naughty = true,
                'q' if prev == 'p' => has_naughty = true,
                'y' if prev == 'x' => has_naughty = true,
                _ => ()
            }
            let prevpair = pairs.get(&(prev, ch)).cloned();
            if let Some(j) = prevpair {
                if j + 1 < i {
                    has_doublepair = true;
                }
            } else {
                pairs.insert((prev, ch), i);
            }
            pprev = prev;
            prev = ch;
        }
        if has_double && vowels >= 3 && !has_naughty {
            nice_rule1 += 1;
        }
        if has_doublepair && has_repeated {
            nice_rule2 += 1;
        }
    }
    println!("Nice strings (rule 1): {}", nice_rule1);
    println!("Nice strings (rule 2): {}", nice_rule2);
}
