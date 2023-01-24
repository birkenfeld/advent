use advtools::prelude::HashSet;
use advtools::input;
use advtools::grid::{Pos, Dir};

type Coords = HashSet<Pos>;

fn walk(directions: impl Iterator<Item=char>, mut set: Coords) -> Coords {
    set.insert(Pos(0, 0));
    set.extend(directions.scan(Pos(0, 0), |xy, ch| {
        Some({ *xy = xy.to(Dir::from_char(ch)); *xy })
    }));
    set
}

fn main() {
    let directions = input::string();
    advtools::verify("# houses", walk(directions.chars(), HashSet::new()).len(), 2565);

    let set = walk(directions.chars().step_by(2), HashSet::new());
    let set = walk(directions.chars().skip(1).step_by(2), set);
    advtools::verify("# houses with robot", set.len(), 2639)
}
