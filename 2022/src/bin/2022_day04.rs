use std::cmp::Ordering::*;
use advtools::input;

fn main() {
    let input = input::rx_lines::<(i32, i32, i32, i32)>(r"(\d+)-(\d+),(\d+)-(\d+)");
    let (contain, overlap) = input.fold((0, 0), |(c, o), (s1, e1, s2, e2)| {
        match s1.cmp(&s2) {
            Equal   => (c + 1, o + 1),
            Less    => (c + (e2 <= e1) as i32, o + (s2 <= e1) as i32),
            Greater => (c + (e1 <= e2) as i32, o + (s1 <= e2) as i32)
        }
    });
    advtools::verify("Fully containing", contain, 513);
    advtools::verify("Overlaps", overlap, 878);
}
