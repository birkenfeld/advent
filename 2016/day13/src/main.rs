use std::collections::HashSet;

const INPUT: i32 = 1352;
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

type Pos = (i32, i32);

fn is_open_space((x, y): Pos) -> bool {
    x >= 0 && y >= 0 && (x*x + 3*x + 2*x*y + y + y*y + INPUT).count_ones() % 2 == 0
}

fn find_steps(initial: Pos, final_: Option<Pos>, limit: usize) -> (Option<usize>, usize) {
    let mut seen = HashSet::with_capacity(1000);
    let mut positions = vec![initial];
    let mut generation = 0;
    let mut reached = None;

    loop {
        generation += 1;
        positions = positions
            .into_iter()
            .flat_map(|(x, y)| {
                DIRECTIONS.iter().filter_map(|&(dx, dy)| {
                    let pos = (x + dx, y + dy);
                    if is_open_space(pos) && !seen.contains(&pos) {
                        seen.insert(pos);
                        if Some(pos) == final_ {
                            reached = Some(generation);
                        }
                        Some(pos)
                    } else { None }
                }).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        if reached.is_some() {
            return (reached, seen.len());
        } else if generation == limit || positions.is_empty() {
            return (None, seen.len());
        }
    }
}

fn main() {
    let pos1 = (1, 1);
    let pos2 = (31, 39);
    println!("Min. # steps to (31,39): {:?}", find_steps(pos1, Some(pos2), 0).0.unwrap());
    println!("Unique locations: {:?}", find_steps(pos1, None, 50).1);
}
