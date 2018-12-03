extern crate advtools;
use advtools::prelude::{HashSet, Itertools};
use advtools::input::input_string;

type Coords = HashSet<(isize, isize)>;

fn walk<I>(directions: I, mut set: Coords) -> Coords where I: IntoIterator<Item=char> {
    set.insert((0, 0));
    set.extend(directions.into_iter().scan((0, 0), |xy, ch| {
        match ch {
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
    let directions = input_string();
    println!("# houses: {}", walk(directions.chars(), HashSet::default()).len());
    let set = walk(directions.chars().step(2), HashSet::default());
    let set = walk(directions.chars().skip(1).step(2), set);
    println!("# houses with robot: {}", set.len())
}
