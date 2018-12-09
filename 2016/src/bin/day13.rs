use advtools::prelude::{HashSet, Itertools};
use advtools::input::{input_string, to_i32};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

type Pos = (i32, i32);

fn is_open_space(input: i32, (x, y): Pos) -> bool {
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
            .flat_map(|(x, y)| {
                DIRECTIONS.iter().filter_map(|(dx, dy)| {
                    let pos = (x + dx, y + dy);
                    if is_open_space(input, pos) && seen.insert(pos) {
                        if Some(pos) == final_ {
                            reached = Some(generation);
                        }
                        Some(pos)
                    } else { None }
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
    let input = to_i32(input_string().trim());
    let pos1 = (1, 1);
    let pos2 = (31, 39);
    advtools::print("Min. # steps to (31,39)", find_steps(input, pos1, Some(pos2), 0).0.unwrap());
    advtools::print("Unique locations", find_steps(input, pos1, None, 50).1);
}
