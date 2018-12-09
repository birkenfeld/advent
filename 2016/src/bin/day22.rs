use advtools::prelude::{HashSet, Regex};
use advtools::input::{iter_input, to_i32};

type Pos = (i32, i32);
const DIRECTIONS: [Pos; 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn is_allowed((x, y): Pos, blockers: &HashSet<Pos>, size: &Pos) -> bool {
    x >= 0 && x <= size.0 && y >= 0 && y <= size.1 && !blockers.contains(&(x, y))
}

fn find_steps(initial: Pos, final_: Pos, blockers: &HashSet<Pos>, size: &Pos) -> usize {
    let mut seen = HashSet::with_capacity(1000);
    let mut positions = vec![initial];
    let mut generation = 0;

    loop {
        generation += 1;
        let mut new_positions = vec![];
        for (x, y) in positions {
            for &(dx, dy) in &DIRECTIONS {
                let new_pos = (x + dx, y + dy);
                if is_allowed(new_pos, blockers, size) && seen.insert(new_pos) {
                    if new_pos == final_ {
                        return generation;
                    }
                    new_positions.push(new_pos);
                }
            }
        }
        positions = new_positions;
    }
}

fn main() {
    let rx = Regex::new(r"node-x(\d+)-y(\d+) +(\d+)T +(\d+)T").unwrap();
    let mut nodes: Vec<(Pos, (i32, i32))> = Vec::new();
    let mut smallest_cap = 1000;
    for line in iter_input::<String>() {
        if let Some(cap) = rx.captures(&line) {
            // (x, y), (size, used)
            nodes.push((
                (to_i32(&cap[1]), to_i32(&cap[2])),
                (to_i32(&cap[3]), to_i32(&cap[4]))
            ));
            smallest_cap = smallest_cap.min((nodes.last().unwrap().1).0);
        }
    }
    let size = nodes[nodes.len() - 1].0;
    let mut blockers = HashSet::new();
    let mut pairs = 0;
    let mut hole_pos = (0, 0);
    for n1 in &nodes {
        // determine if it's a blocker
        if (n1.1).1 > smallest_cap {
            blockers.insert(n1.0);
        }

        // determine initial hole
        if (n1.1).1 == 0 {
            hole_pos = ((n1.0).0 as i32, (n1.0).1 as i32);
        }

        // determine viable pairs
        for n2 in &nodes {
            if n1.0 == n2.0 || (n1.1).1 == 0 {
                continue;
            }
            if (n1.1).1 <= ((n2.1).0 - (n2.1).1) {
                pairs += 1;
            }
        }
    }
    advtools::print("Number of viable pairs", pairs);

    let mut total_steps = 0;
    // move data from (max,0) to (0,0) step by step
    for target_x in (0 .. size.0).rev() {
        // find shortest way for the hole to move to (tx,0),
        // temporarily blocking the data position (it may not move!)
        blockers.insert((target_x + 1, 0));
        let steps = find_steps(hole_pos, (target_x as i32, 0), &blockers, &size);
        blockers.take(&(target_x + 1, 0));

        // add one additional step (moving the data into the hole)
        total_steps += steps + 1;
        hole_pos = (target_x as i32 + 1, 0);
    }
    advtools::print("Total # of steps", total_steps);
}
