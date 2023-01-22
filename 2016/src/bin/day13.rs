use advtools::prelude::{HashSet, Itertools};
use advtools::input;
use advtools::grid::Pos;

fn is_open_space(input: i32, Pos { x, y }: Pos) -> bool {
    x >= 0 && y >= 0 && (x*x + 3*x + 2*x*y + y + y*y + input).count_ones() % 2 == 0
}

fn find_steps(input: i32, initial: Pos, final_: Option<Pos>, limit: usize)
              -> (Option<usize>, usize) {
    let mut seen = HashSet::with_capacity(1000);
    let mut positions = vec![initial];
    let mut generation = 0;
    let mut reached = None;

    loop {
        generation += 1;
        positions = positions
            .into_iter()
            .flat_map(|pos| {
                pos.neighbors().filter(|&new_pos| {
                    if is_open_space(input, new_pos) && seen.insert(new_pos) {
                        if Some(new_pos) == final_ {
                            reached = Some(generation);
                        }
                        true
                    } else { false }
                }).collect_vec()
            })
            .collect_vec();
        if reached.is_some() {
            return (reached, seen.len());
        } else if generation == limit || positions.is_empty() {
            return (None, seen.len());
        }
    }
}

fn main() {
    let input = input::parse();
    let pos1 = Pos(1, 1);
    let pos2 = Pos(31, 39);
    advtools::verify("Min. # steps to (31,39)", find_steps(input, pos1, Some(pos2), 0).0.unwrap(), 90);
    advtools::verify("Unique locations", find_steps(input, pos1, None, 50).1, 135);
}
