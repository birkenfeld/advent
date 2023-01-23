use std::fmt::Write;
use advtools::prelude::{Itertools, HashSet};
use advtools::input;

const FORMAT: &str = r"(\d+),(\d+)|fold along (.)=(\d+)";

fn main() {
    // Parse the input.
    let mut dots = HashSet::new();
    let mut folds = Vec::new();
    for (coords, fold) in input::rx_lines::<(Option<(u32, u32)>, Option<(&str, u32)>)>(FORMAT) {
        if let Some(fold) = fold {
            folds.push(fold);
        } else if let Some(coords) = coords {
            dots.insert(coords);
        }
    }

    // Perform the folds.
    let mut max_count = 0;
    for (dir, axis) in folds {
        dots = if dir == "x" {
            dots.into_iter().map(|(x, y)| (x.min(2*axis - x), y)).collect()
        } else {
            dots.into_iter().map(|(x, y)| (x, y.min(2*axis - y))).collect()
        };
        max_count = max_count.max(dots.len());
    }
    advtools::verify("Dots after first fold", max_count, 724);

    // Determine the extent of the picture and print it.
    let (xmin, xmax) = dots.iter().map(|k| k.0).minmax().into_option().unwrap();
    let (ymin, ymax) = dots.iter().map(|k| k.1).minmax().into_option().unwrap();
    let mut out = String::new();
    for y in ymin..=ymax {
        writeln!(out).unwrap();
        for x in xmin..=xmax {
            let px = if dots.contains(&(x, y)) { "█" } else { " " };
            write!(out, "{}", px).unwrap();
        }
    }
    advtools::print("Code letters", out);
}
