#![feature(io)]

extern crate itertools;

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;

type Coords = HashSet<(usize, usize)>;

fn walk<'a, I>(directions: I, mut set: Coords) -> Coords where I: IntoIterator<Item=&'a char> {
    set.insert((0, 0));
    set.extend(directions.into_iter().scan((0, 0), |xy, ch| {
        match *ch {
            '<' => xy.0 -= 1,
            '>' => xy.0 += 1,
            'v' => xy.1 -= 1,
            '^' => xy.1 += 1,
            _ => ()
        }
        Some(*xy)
    }));
    set
}

fn main() {
    let fp = File::open("input.txt").unwrap();
    let directions = fp.chars().collect::<Result<Vec<char>, _>>().unwrap();
    println!("# houses: {}", walk(&directions, HashSet::new()).len());
    let set = walk(directions.iter().step(2), HashSet::new());
    let set = walk(directions.iter().skip(1).step(2), set);
    println!("# houses with robot: {}", set.len())
}
