use advtools::prelude::HashSet;
use advtools::input::input_string;

type Coords = HashSet<(i32, i32)>;

fn walk(directions: impl Iterator<Item=char>, mut set: Coords) -> Coords {
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
    advtools::verify("# houses", walk(directions.chars(), HashSet::new()).len(), 2565);

    let set = walk(directions.chars().step_by(2), HashSet::new());
    let set = walk(directions.chars().skip(1).step_by(2), set);
    advtools::verify("# houses with robot", set.len(), 2639)
}
