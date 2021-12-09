use advtools::prelude::HashMap;
use advtools::input::iter_input_regex;

// Find the overlapping squares with or without diagonal lines.
fn overlaps(input: impl Iterator<Item=(i32, i32, i32, i32)>, do_diag: bool) -> usize {
    let mut squares = HashMap::<_, i32>::new();
    for (x1, y1, x2, y2) in input {
        // Find the direction in x and y.
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        // The number of steps is given by x or y
        let steps = if dx != 0 { x1 - x2 } else { y1 - y2 }.abs();

        // Record line if it's horizontal or vertical, or we want diagonals.
        if dx == 0 || dy == 0 || do_diag {
            for i in 0..=steps {
                *squares.entry((x1 + i*dx, y1 + i*dy)).or_default() += 1;
            }
        }
    }
    // Overlapping are all squares where we visited more than once.
    squares.values().filter(|&&v| v > 1).count()
}

fn main() {
    let p1 = overlaps(iter_input_regex(r"(\d+),(\d+) -> (\d+),(\d+)"), false);
    let p2 = overlaps(iter_input_regex(r"(\d+),(\d+) -> (\d+),(\d+)"), true);

    advtools::verify("Overlaps", p1, 4421);
    advtools::verify("With diagonals", p2, 18674);
}
