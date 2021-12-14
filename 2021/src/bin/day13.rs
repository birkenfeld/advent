use std::fmt::Write;
use advtools::prelude::{Itertools, HashSet};
use advtools::input::{iter_lines, to_usize};

fn main() {
    // Parse the input.
    let mut dots = HashSet::new();
    let mut folds = Vec::new();
    for line in iter_lines() {
        if line.starts_with("fold along x=") {
            folds.push((0, to_usize(&line[13..])));
        } else if line.starts_with("fold along y") {
            folds.push((1, to_usize(&line[13..])));
        } else {
            let (x, y) = line.split(',').map(to_usize).collect_tuple().unwrap();
            dots.insert((x, y));
        }
    }

    // Perform the folds.
    let mut max_count = 0;
    for (dir, axis) in folds {
        dots = if dir == 0 {
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
