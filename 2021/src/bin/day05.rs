use advtools::prelude::HashSet;
use advtools::input::iter_input_regex;

fn overlaps(input: impl Iterator<Item=(i32, i32, i32, i32)>, do_diag: bool) -> usize {
    let mut squares = HashSet::new();
    let mut overlaps = HashSet::new();
    for (x1, y1, x2, y2) in input {
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        let steps = (x1 - x2).abs().max((y1 - y2).abs());

        if dx == 0 || dy == 0 || do_diag {
            for i in 0..=steps {
                let coords = (x1 + i*dx, y1 + i*dy);
                if !squares.insert(coords) {
                    overlaps.insert(coords);
                }
            }
        }
    }
    overlaps.len()
}

fn main() {
    let p1 = overlaps(iter_input_regex(r"(\d+),(\d+) -> (\d+),(\d+)"), false);
    let p2 = overlaps(iter_input_regex(r"(\d+),(\d+) -> (\d+),(\d+)"), true);

    advtools::verify("Overlaps", p1, 4421);
    advtools::verify("With diagonals", p2, 18674);
}
