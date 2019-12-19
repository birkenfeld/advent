use advtools::prelude::{Itertools, HashMap, FromIterator};
use advtools::input::input_string;
use advent19::Machine;
use num::Zero;

#[derive(Clone, PartialEq)]
enum Color { Black, White }
enum Dir { U, L, D, R }
use {Dir::*, Color::*};

impl Dir {
    fn ccw(self) -> Self {
        match self { U => L, L => D, D => R, R => U }
    }

    fn cw(self) -> Self {
        match self { U => R, R => D, D => L, L => U }
    }
}

fn main() {
    let code = Machine::parse(&input_string());
    let mut machine = Machine::new(&code);

    let mut walk = |tiles: &mut HashMap<(i32, i32), Color>| {
        let (mut x, mut y, mut dir) = (0, 0, U);
        loop {
            // Determine current color and feed it to the machine.
            let cur_color = tiles.get(&(x, y)).cloned().unwrap_or(Black);
            let paint = match machine.run(Some(cur_color as u8)) {
                Some(p) => if p.is_zero() { Black } else { White },
                None => break
            };
            // Mark this tile as painted, white or black.
            tiles.insert((x, y), paint);
            // Determine direction, turn and advance one step.
            dir = if machine.next().unwrap().is_zero() { dir.ccw() } else { dir.cw() };
            match dir {
                U => y -= 1,
                L => x -= 1,
                D => y += 1,
                R => x += 1,
            }
        }
    };

    // Walk from a black tile.
    let mut tiles = HashMap::new();
    walk(&mut tiles);
    advtools::print("Painted tiles", tiles.len());

    // Walk from a white tile.
    tiles = HashMap::from_iter(Some(((0, 0), White)));
    walk(&mut tiles);

    // Determine the extent of the picture and print it.
    let (xmin, xmax) = tiles.keys().map(|k| k.0).minmax().into_option().unwrap();
    let (ymin, ymax) = tiles.keys().map(|k| k.1).minmax().into_option().unwrap();
    advtools::print("Registration identifier", "");
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            print!("{}", if tiles.get(&(x, y)) == Some(&White) { "█" } else { " " });
        }
        println!();
    }
}
