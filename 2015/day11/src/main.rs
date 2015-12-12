fn increment(pw: &mut Vec<u8>) {
    for i in (0..8).rev() {
        if pw[i] < b'z' {
            pw[i] += 1;
            break;
        } else {
            pw[i] = b'a';
        }
    }
}

fn main() {
    let mut pw = b"vzbxkghb".to_vec();
    let mut found = 0;
    'outer: loop {
        increment(&mut pw);
        let mut pch = 0;
        let mut ppch = 0;
        let mut has_run = false;
        let mut pairs = 0;
        for &ch in &pw {
            if ch == b'i' || ch == b'o' || ch == b'l' {
                continue 'outer;
            }
            if ch == pch && pch != ppch {
                pairs += 1;
            }
            if ppch + 1 == pch && pch + 1 == ch {
                has_run = true;
            }
            ppch = pch;
            pch = ch;
        }
        if has_run && pairs >= 2 {
            found += 1;
            println!("Next password: {}", String::from_utf8(pw.clone()).unwrap());
            if found == 2 {
                break;
            }
        }
    }
}
