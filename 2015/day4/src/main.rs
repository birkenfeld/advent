extern crate openssl;

use std::io::Write;
use openssl::crypto::hash::{Type, Hasher};

fn main() {
    let mut init_hasher = Hasher::new(Type::MD5);
    init_hasher.write(b"yzbqklnj").unwrap();
    for i in 0..10000000 {
        let mut hasher = init_hasher.clone();
        write!(hasher, "{}", i).unwrap();
        let h = hasher.finish();
        if h[0] | h[1] == 0 {
            if h[2] & 0xF0 == 0 {
                println!("Found 5-zero hash: {}", i);
            }
            if h[2] == 0 {
                println!("Found 6-zero hash: {}", i);
                break;
            }
        }
    }
}
