extern crate advtools;
use advtools::prelude::Regex;
use advtools::input::{input_string, to_usize};

fn get_decompressed_length(mut s: &str, rx: &Regex, recursive: bool) -> usize {
    let mut size = 0;
    while let Some(cap) = rx.captures(s) {
        let nchars = to_usize(&cap[1]);
        let repetition = to_usize(&cap[2]);
        let span = cap.get(0).unwrap();
        size += span.start() + repetition * if !recursive { nchars } else {
            get_decompressed_length(&s[span.end()..span.end()+nchars], rx, true)
        };
        s = &s[span.end()+nchars..];
    }
    size + s.len()
}

fn main() {
    let rx = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    let input = input_string().replace('\n', "");
    println!("Length of data (v1): {}", get_decompressed_length(&input, &rx, false));
    println!("Length of data (v2): {}", get_decompressed_length(&input, &rx, true));
}
