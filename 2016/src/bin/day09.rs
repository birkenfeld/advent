use advtools::prelude::Regex;
use advtools::input;

fn get_decompressed_length(mut s: &str, rx: &Regex, recursive: bool) -> usize {
    let mut size = 0;
    while let Some(cap) = rx.captures(s) {
        let nchars = cap[1].parse().unwrap();
        let repetition: usize = cap[2].parse().unwrap();
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
    let input = input::string();
    advtools::verify("Length of data (v1)", get_decompressed_length(input, &rx, false), 110346);
    advtools::verify("Length of data (v2)", get_decompressed_length(input, &rx, true), 10774309173_u64);
}
