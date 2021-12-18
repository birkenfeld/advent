use advtools::prelude::Itertools;
use advtools::input;

fn main() {
    let first_line = input::lines().next().unwrap();
    let mut arrs = vec![[0; 26]; first_line.len()];

    for line in input::lines() {
        for (arr, ch) in arrs.iter_mut().zip(line.chars()) {
            arr[(ch as u8 - b'a') as usize] += 1;
        }
    }

    let collect_by_freq = |weight| arrs.iter().map(|arr| {
        let mut freqs = arr.iter().enumerate().map(|(i, v)| (weight * v, i)).sorted();
        (freqs.next().unwrap().1 as u8 + b'a') as char
    }).collect::<String>();

    advtools::verify("Message (most common)", collect_by_freq(-1), "cyxeoccr");
    advtools::verify("Message (least common)", collect_by_freq(1), "batwpask");
}
