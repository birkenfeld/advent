use std::fmt::Write;
use advtools::prelude::{Itertools, HashMap};
use advtools::input;
use advtools::grid::{Pos, Dir};
use advent19::Machine;

#[derive(Clone, PartialEq)]
enum Color { Black, White }
use Color::*;

fn main() {
    let code = Machine::parse(input::string());
    let mut machine = Machine::new(&code);

    let mut walk = |tiles: &mut HashMap<Pos, Color>| {
        let (mut pos, mut dir) = (Pos(0, 0), Dir::U);
        loop {
            // Determine current color and feed it to the machine.
            let cur_color = tiles.get(&pos).cloned().unwrap_or(Black);
            let paint = match machine.next_with(cur_color as i64) {
                Some(p) => if p == 0 { Black } else { White },
                None => break
            };
            // Mark this tile as painted, white or black.
            tiles.insert(pos, paint);
            // Determine direction, turn and advance one step.
            dir = if machine.next().unwrap() == 0 { dir.left() } else { dir.right() };
            pos.step(dir);
        }
    };

    // Walk from a black tile.
    let mut tiles = HashMap::new();
    walk(&mut tiles);
    advtools::verify("Painted tiles", tiles.len(), 1709);

    // Walk from a white tile.
    tiles = std::iter::once((Pos(0, 0), White)).collect();
    walk(&mut tiles);

    // Determine the extent of the picture and print it.
    let (xmin, xmax) = tiles.keys().map(|k| k.x).minmax().into_option().unwrap();
    let (ymin, ymax) = tiles.keys().map(|k| k.y).minmax().into_option().unwrap();
    let mut out = String::new();
    for y in ymin..=ymax {
        writeln!(out).unwrap();
        for x in xmin..=xmax {
            let px = if tiles.get(&Pos(x, y)) == Some(&White) { "█" } else { " " };
            write!(out, "{}", px).unwrap();
        }
    }
    advtools::print("Registration identifier", out);
}
