use std::fmt::Write;
use advtools::prelude::{Itertools, HashMap};
use advtools::input::input_string;
use advent19::{Machine, Dir};

#[derive(Clone, PartialEq)]
enum Color { Black, White }
use Color::*;

fn main() {
    let code = Machine::parse(&input_string());
    let mut machine = Machine::new(&code);

    let mut walk = |tiles: &mut HashMap<(i32, i32), Color>| {
        let (mut xy, mut dir) = ((0, 0), Dir::U);
        loop {
            // Determine current color and feed it to the machine.
            let cur_color = tiles.get(&xy).cloned().unwrap_or(Black);
            let paint = match machine.next_with(cur_color as i64) {
                Some(p) => if p == 0 { Black } else { White },
                None => break
            };
            // Mark this tile as painted, white or black.
            tiles.insert(xy, paint);
            // Determine direction, turn and advance one step.
            dir = if machine.next().unwrap() == 0 { dir.left() } else { dir.right() };
            xy = dir.step(xy);
        }
    };

    // Walk from a black tile.
    let mut tiles = HashMap::new();
    walk(&mut tiles);
    advtools::verify("Painted tiles", tiles.len(), 1709);

    // Walk from a white tile.
    tiles = std::iter::once(((0, 0), White)).collect();
    walk(&mut tiles);

    // Determine the extent of the picture and print it.
    let (xmin, xmax) = tiles.keys().map(|k| k.0).minmax().into_option().unwrap();
    let (ymin, ymax) = tiles.keys().map(|k| k.1).minmax().into_option().unwrap();
    let mut out = String::new();
    for y in ymin..=ymax {
        writeln!(out).unwrap();
        for x in xmin..=xmax {
            write!(out, "{}", if tiles.get(&(x, y)) == Some(&White) { "█" } else { " " }).unwrap();
        }
    }
    advtools::print("Registration identifier", out);
}
