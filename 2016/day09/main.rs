extern crate advtools;
extern crate regex;

fn get_decompressed_length(mut s: &str, rx: &regex::Regex, recursive: bool) -> usize {
    let mut size = 0;
    while let Some(cap) = rx.captures(s) {
        let nchars: usize = cap[1].parse().unwrap();
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
    let rx = regex::Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    let input = advtools::input_string().replace('\n', "");
    println!("Length of data (v1): {}", get_decompressed_length(&input, &rx, false));
    println!("Length of data (v2): {}", get_decompressed_length(&input, &rx, true));
}
