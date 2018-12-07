use advtools::prelude::Itertools;
use advtools::input::from_utf8;

const INPUT: &[u8] = b"vzbxkghb";

fn increment(pw: &mut [u8]) {
    for ch in pw.iter_mut().rev() {
        if *ch < b'z' {
            *ch += 1;
            break;
        } else {
            *ch = b'a';
        }
    }
}

fn is_ok(pw: &[u8]) -> bool {
    pw.iter().tuple_windows().filter(|(c1, c2, c3)| c1 != c2 && c2 == c3).count() >= 2 &&
        pw.iter().tuple_windows().any(|(c1, c2, c3)| c1 + 1 == *c2 && c2 + 1 == *c3) &&
        !pw.iter().any(|&ch| ch == b'i' || ch == b'o' || ch == b'l')
}

fn main() {
    let mut pw = INPUT.to_vec();
    let mut found = 0;
    loop {
        increment(&mut pw);
        if is_ok(&pw) {
            found += 1;
            println!("Next password: {}", from_utf8(&pw));
            if found == 2 {
                break;
            }
        }
    }
}
