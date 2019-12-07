use advtools::prelude::HashSet;
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
    advtools::print("# houses", walk(directions.chars(), HashSet::new()).len());
    let set = walk(directions.chars().step_by(2), HashSet::new());
    let set = walk(directions.chars().skip(1).step_by(2), set);
    advtools::print("# houses with robot", set.len())
}
